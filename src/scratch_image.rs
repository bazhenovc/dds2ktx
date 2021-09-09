// Copyright (c) 2021 Kyrylo Bazhenov
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::dds::*;

#[derive(Clone)]
pub struct ScratchImage {
    dds_header: DirectDrawHeader,
    dds_data: Vec<u8>,
}

impl ScratchImage {
    pub fn from_file(path: &std::path::Path) -> ScratchImage {
        use std::io::Read;

        let mut dds_file = std::fs::File::open(path).expect("failed to open dds file");
        let dds_header = {
            let mut header_bytes = [0u8; 148];
            dds_file
                .read_exact(&mut header_bytes)
                .expect("failed to read dds header");

            let header: &DirectDrawHeader = bytemuck::from_bytes(&header_bytes);

            assert_eq!(&header.magic, b"DDS ", "DDS magic mismatch");
            assert_eq!(header.size, 124, "DDS header size mismatch");
            assert_eq!(header.pixel_format.size, 32, "DDS pixel format size mismatch");
            assert_eq!(&header.pixel_format.four_cc, b"DX10", "DX9 files not implemented yet");

            *header
        };
        let dds_data = {
            let mut buffer = Vec::new();
            dds_file.read_to_end(&mut buffer).expect("failed to read dds data");
            buffer
        };

        let is_compressed = is_block_compressed(dds_header.dxt10.dxgi_format);
        let (row_pitch, linear_size) =
            pitch_and_linear_size(dds_header.width, dds_header.height, dds_header.dxt10.dxgi_format);
        if is_compressed {
            assert_eq!(linear_size, dds_header.pitch_or_linear_size);
        } else {
            assert_eq!(row_pitch, dds_header.pitch_or_linear_size);
        }

        let mut image_data_size = linear_size;
        for mip in 1..dds_header.mipmap_count {
            let (_, mip_linear_size) = pitch_and_linear_size(
                dds_header.width >> mip,
                dds_header.height >> mip,
                dds_header.dxt10.dxgi_format,
            );
            image_data_size += mip_linear_size;
        }
        if dds_header.dxt10.misc_flag & DDS_RESOURCE_MISC_TEXTURECUBE == DDS_RESOURCE_MISC_TEXTURECUBE {
            image_data_size *= 6;
        }
        assert_eq!(image_data_size, dds_data.len() as _);

        ScratchImage { dds_header, dds_data }
    }

    pub fn new(
        width: u32,
        height: u32,
        depth: u32,
        mipmap_count: u32,
        array_size: u32,
        dxgi_format: u32,
        is_cubemap: bool,
    ) -> ScratchImage {
        let mut flags = DDSD_CAPS | DDSD_PIXELFORMAT;
        let mut resource_dimension = D3D10_RESOURCE_DIMENSION_UNKNOWN;
        let mut caps = DDSCAPS_TEXTURE;
        let mut caps2 = 0;

        if is_block_compressed(dxgi_format) {
            flags |= DDSD_LINEARSIZE;
        }

        if width > 1 {
            flags |= DDSD_WIDTH;
            resource_dimension = D3D10_RESOURCE_DIMENSION_TEXTURE1D;
        }
        if height > 1 {
            flags |= DDSD_HEIGHT;
            resource_dimension = D3D10_RESOURCE_DIMENSION_TEXTURE2D;
        }
        if depth > 1 {
            flags |= DDSD_DEPTH;
            resource_dimension = D3D10_RESOURCE_DIMENSION_TEXTURE3D;
            caps |= DDSCAPS_COMPLEX;
            caps2 |= DDSCAPS2_VOLUME;
        }
        if mipmap_count > 1 {
            flags |= DDSD_MIPMAPCOUNT;
            caps |= DDSCAPS_MIPMAP;
            caps |= DDSCAPS_COMPLEX;
        }

        let mut misc_flag = 0;
        if is_cubemap {
            caps |= DDSCAPS_COMPLEX;
            caps2 |= DDSCAPS2_CUBEMAP;
            caps2 |= DDSCAPS2_CUBEMAP_POSITIVEX;
            caps2 |= DDSCAPS2_CUBEMAP_NEGATIVEX;
            caps2 |= DDSCAPS2_CUBEMAP_POSITIVEY;
            caps2 |= DDSCAPS2_CUBEMAP_NEGATIVEY;
            caps2 |= DDSCAPS2_CUBEMAP_POSITIVEZ;
            caps2 |= DDSCAPS2_CUBEMAP_NEGATIVEZ;
            misc_flag |= DDS_RESOURCE_MISC_TEXTURECUBE
        }
        let (row_pitch, linear_size) = pitch_and_linear_size(width, height, dxgi_format);

        let dds_header = DirectDrawHeader {
            magic: *b"DDS ",
            size: 124,
            flags,
            height,
            width,
            pitch_or_linear_size: if is_block_compressed(dxgi_format) {
                linear_size
            } else {
                row_pitch
            },
            depth,
            mipmap_count,
            reserved: [0; 11],
            pixel_format: DirectDrawPixelFormat {
                size: 32,
                flags: DDPF_FOURCC,
                four_cc: *b"DX10",
                rgb_bit_count: 0,
                red_bit_mask: 0,
                green_bit_mask: 0,
                blue_bit_mask: 0,
                alpha_bit_mask: 0,
            },
            caps,
            caps2,
            caps3: 0,
            caps4: 0,
            reserved2: 0,
            dxt10: DirectDrawHeader10 {
                dxgi_format,
                resource_dimension,
                misc_flag,
                array_size,
                misc_flags2: 0,
            },
        };

        let mut image_data_size = linear_size;
        for mip in 1..mipmap_count {
            let (_, mip_linear_size) = pitch_and_linear_size(width >> mip, height >> mip, dxgi_format);
            image_data_size += mip_linear_size;
        }
        if is_cubemap {
            image_data_size *= 6;
        }

        let mut dds_data = Vec::new();
        dds_data.resize(image_data_size as _, 0u8);

        ScratchImage { dds_header, dds_data }
    }

    pub fn save_to_file(&self, path: &std::path::Path) {
        let header = bytemuck::bytes_of(&self.dds_header);

        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .expect("failed to open dds output file");

        file.write_all(&header).expect("failed to write dds header");
        file.write_all(&self.dds_data[..]).expect("failed to write dds data");
    }

    pub fn image_size(&self) -> (u32, u32, u32) {
        (self.dds_header.width, self.dds_header.height, self.dds_header.depth)
    }

    pub fn image_width(&self) -> u32 {
        self.dds_header.width
    }

    pub fn image_height(&self) -> u32 {
        self.dds_header.height
    }

    pub fn image_depth(&self) -> u32 {
        self.dds_header.depth
    }

    pub fn mipmap_count(&self) -> u32 {
        self.dds_header.mipmap_count
    }

    pub fn layer_count(&self) -> u32 {
        self.dds_header.dxt10.array_size
    }

    pub fn block_size(&self) -> u32 {
        block_size(self.dds_header.dxt10.dxgi_format)
    }

    pub fn is_texture1d(&self) -> bool {
        self.dds_header.dxt10.resource_dimension == D3D10_RESOURCE_DIMENSION_TEXTURE1D
    }

    pub fn is_texture2d(&self) -> bool {
        self.dds_header.dxt10.resource_dimension == D3D10_RESOURCE_DIMENSION_TEXTURE2D
    }

    pub fn is_texture3d(&self) -> bool {
        self.dds_header.dxt10.resource_dimension == D3D10_RESOURCE_DIMENSION_TEXTURE3D
    }

    pub fn is_cubemap(&self) -> bool {
        self.dds_header.dxt10.misc_flag & DDS_RESOURCE_MISC_TEXTURECUBE == DDS_RESOURCE_MISC_TEXTURECUBE
    }

    pub fn dxgi_format(&self) -> u32 {
        self.dds_header.dxt10.dxgi_format
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.dds_data
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.dds_data
    }

    pub fn as_typed_slice<T>(&self) -> &[T] {
        let source_size = bits_per_pixel(self.dds_header.dxt10.dxgi_format) / 8;
        let target_size = std::mem::size_of::<T>();
        assert_eq!(source_size, target_size as _);

        unsafe { std::slice::from_raw_parts(self.dds_data.as_ptr() as *const T, self.dds_data.len() * target_size) }
    }

    pub fn as_typed_slice_mut<T>(&mut self) -> &mut [T] {
        let source_size = bits_per_pixel(self.dds_header.dxt10.dxgi_format) / 8;
        let target_size = std::mem::size_of::<T>();
        assert_eq!(source_size, target_size as _);

        unsafe {
            std::slice::from_raw_parts_mut(self.dds_data.as_mut_ptr() as *mut T, self.dds_data.len() * target_size)
        }
    }
}
