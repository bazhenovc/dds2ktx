// Copyright (c) 2021 Kyrylo Bazhenov
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code)]

mod dds;

mod scratch_image;
use scratch_image::*;

mod vk_format;
use vk_format::*;

mod gl_format;
use gl_format::*;

use libktx_rs::{sinks::*, sources::*, *};
use structopt::*;

#[derive(Debug, StructOpt)]
struct CommandLineOptions {
    #[structopt(short = "i", long = "input", help = "Input .dds texture path", parse(from_os_str))]
    input_file: std::path::PathBuf,

    #[structopt(short = "o", long = "output", help = "Output .ktx texture path", parse(from_os_str))]
    output_file: std::path::PathBuf,

    #[structopt(long = "ktx2", help = "Use KTX 2 format for writing")]
    ktx2: bool,
}

fn main() {
    let command_line = CommandLineOptions::from_args();

    let input_image = ScratchImage::from_file(&command_line.input_file);

    let common = CommonCreateInfo {
        create_storage: CreateStorage::AllocStorage,
        base_width: input_image.image_width(),
        base_height: input_image.image_height(),
        base_depth: input_image.image_depth(),
        num_dimensions: if input_image.is_texture1d() {
            1
        } else if input_image.is_texture3d() {
            3
        } else {
            2
        },
        num_levels: input_image.mipmap_count(),
        num_layers: input_image.layer_count(),
        num_faces: if input_image.is_cubemap() { 6 } else { 1 },
        is_array: input_image.layer_count() > 1,
        generate_mipmaps: false,
    };
    let mut output_image = if command_line.ktx2 {
        Texture::new(Ktx2CreateInfo {
            vk_format: dxgi_to_vk(input_image.dxgi_format()),
            dfd: None,
            common,
        })
        .expect("Failed to create output image")
    } else {
        Texture::new(Ktx1CreateInfo {
            gl_internal_format: dxgi_to_gl(input_image.dxgi_format()),
            common,
        })
        .expect("Failed to create output image")
    };

    let mut source_slice = input_image.as_slice();
    output_image
        .iterate_levels_mut(|_miplevel, _face, _width, _height, _depth, pixel_data| {
            let (src, remaining) = source_slice.split_at(pixel_data.len());
            source_slice = remaining;
            pixel_data.copy_from_slice(&src);
            Ok(())
        })
        .expect("Failed to iterate ktx levels");

    if !source_slice.is_empty() {
        panic!(
            "Not all data was copied to ktx image, remaining bytes: {}",
            source_slice.len()
        );
    }

    let output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(command_line.output_file)
        .expect("Failed to open output file");

    let ktx_stream = RustKtxStream::new(Box::new(output_file)).expect("Failed to create ktx stream");
    let mut ktx_sink = StreamSink::new(std::sync::Arc::new(std::sync::Mutex::new(ktx_stream)));
    output_image.write_to(&mut ktx_sink).expect("Failed to write ktx image");
}
