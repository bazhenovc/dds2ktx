// Copyright (c) 2021 Kyrylo Bazhenov
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code, non_upper_case_globals)]

pub const VK_FORMAT_UNDEFINED: u32 = 0;
pub const VK_FORMAT_R4G4_UNORM_PACK8: u32 = 1;
pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: u32 = 2;
pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: u32 = 3;
pub const VK_FORMAT_R5G6B5_UNORM_PACK16: u32 = 4;
pub const VK_FORMAT_B5G6R5_UNORM_PACK16: u32 = 5;
pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: u32 = 6;
pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: u32 = 7;
pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: u32 = 8;
pub const VK_FORMAT_R8_UNORM: u32 = 9;
pub const VK_FORMAT_R8_SNORM: u32 = 10;
pub const VK_FORMAT_R8_USCALED: u32 = 11;
pub const VK_FORMAT_R8_SSCALED: u32 = 12;
pub const VK_FORMAT_R8_UINT: u32 = 13;
pub const VK_FORMAT_R8_SINT: u32 = 14;
pub const VK_FORMAT_R8_SRGB: u32 = 15;
pub const VK_FORMAT_R8G8_UNORM: u32 = 16;
pub const VK_FORMAT_R8G8_SNORM: u32 = 17;
pub const VK_FORMAT_R8G8_USCALED: u32 = 18;
pub const VK_FORMAT_R8G8_SSCALED: u32 = 19;
pub const VK_FORMAT_R8G8_UINT: u32 = 20;
pub const VK_FORMAT_R8G8_SINT: u32 = 21;
pub const VK_FORMAT_R8G8_SRGB: u32 = 22;
pub const VK_FORMAT_R8G8B8_UNORM: u32 = 23;
pub const VK_FORMAT_R8G8B8_SNORM: u32 = 24;
pub const VK_FORMAT_R8G8B8_USCALED: u32 = 25;
pub const VK_FORMAT_R8G8B8_SSCALED: u32 = 26;
pub const VK_FORMAT_R8G8B8_UINT: u32 = 27;
pub const VK_FORMAT_R8G8B8_SINT: u32 = 28;
pub const VK_FORMAT_R8G8B8_SRGB: u32 = 29;
pub const VK_FORMAT_B8G8R8_UNORM: u32 = 30;
pub const VK_FORMAT_B8G8R8_SNORM: u32 = 31;
pub const VK_FORMAT_B8G8R8_USCALED: u32 = 32;
pub const VK_FORMAT_B8G8R8_SSCALED: u32 = 33;
pub const VK_FORMAT_B8G8R8_UINT: u32 = 34;
pub const VK_FORMAT_B8G8R8_SINT: u32 = 35;
pub const VK_FORMAT_B8G8R8_SRGB: u32 = 36;
pub const VK_FORMAT_R8G8B8A8_UNORM: u32 = 37;
pub const VK_FORMAT_R8G8B8A8_SNORM: u32 = 38;
pub const VK_FORMAT_R8G8B8A8_USCALED: u32 = 39;
pub const VK_FORMAT_R8G8B8A8_SSCALED: u32 = 40;
pub const VK_FORMAT_R8G8B8A8_UINT: u32 = 41;
pub const VK_FORMAT_R8G8B8A8_SINT: u32 = 42;
pub const VK_FORMAT_R8G8B8A8_SRGB: u32 = 43;
pub const VK_FORMAT_B8G8R8A8_UNORM: u32 = 44;
pub const VK_FORMAT_B8G8R8A8_SNORM: u32 = 45;
pub const VK_FORMAT_B8G8R8A8_USCALED: u32 = 46;
pub const VK_FORMAT_B8G8R8A8_SSCALED: u32 = 47;
pub const VK_FORMAT_B8G8R8A8_UINT: u32 = 48;
pub const VK_FORMAT_B8G8R8A8_SINT: u32 = 49;
pub const VK_FORMAT_B8G8R8A8_SRGB: u32 = 50;
pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: u32 = 51;
pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: u32 = 52;
pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: u32 = 53;
pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: u32 = 54;
pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: u32 = 55;
pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: u32 = 56;
pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: u32 = 57;
pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: u32 = 58;
pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: u32 = 59;
pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: u32 = 60;
pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: u32 = 61;
pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: u32 = 62;
pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: u32 = 63;
pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: u32 = 64;
pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: u32 = 65;
pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: u32 = 66;
pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: u32 = 67;
pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: u32 = 68;
pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: u32 = 69;
pub const VK_FORMAT_R16_UNORM: u32 = 70;
pub const VK_FORMAT_R16_SNORM: u32 = 71;
pub const VK_FORMAT_R16_USCALED: u32 = 72;
pub const VK_FORMAT_R16_SSCALED: u32 = 73;
pub const VK_FORMAT_R16_UINT: u32 = 74;
pub const VK_FORMAT_R16_SINT: u32 = 75;
pub const VK_FORMAT_R16_SFLOAT: u32 = 76;
pub const VK_FORMAT_R16G16_UNORM: u32 = 77;
pub const VK_FORMAT_R16G16_SNORM: u32 = 78;
pub const VK_FORMAT_R16G16_USCALED: u32 = 79;
pub const VK_FORMAT_R16G16_SSCALED: u32 = 80;
pub const VK_FORMAT_R16G16_UINT: u32 = 81;
pub const VK_FORMAT_R16G16_SINT: u32 = 82;
pub const VK_FORMAT_R16G16_SFLOAT: u32 = 83;
pub const VK_FORMAT_R16G16B16_UNORM: u32 = 84;
pub const VK_FORMAT_R16G16B16_SNORM: u32 = 85;
pub const VK_FORMAT_R16G16B16_USCALED: u32 = 86;
pub const VK_FORMAT_R16G16B16_SSCALED: u32 = 87;
pub const VK_FORMAT_R16G16B16_UINT: u32 = 88;
pub const VK_FORMAT_R16G16B16_SINT: u32 = 89;
pub const VK_FORMAT_R16G16B16_SFLOAT: u32 = 90;
pub const VK_FORMAT_R16G16B16A16_UNORM: u32 = 91;
pub const VK_FORMAT_R16G16B16A16_SNORM: u32 = 92;
pub const VK_FORMAT_R16G16B16A16_USCALED: u32 = 93;
pub const VK_FORMAT_R16G16B16A16_SSCALED: u32 = 94;
pub const VK_FORMAT_R16G16B16A16_UINT: u32 = 95;
pub const VK_FORMAT_R16G16B16A16_SINT: u32 = 96;
pub const VK_FORMAT_R16G16B16A16_SFLOAT: u32 = 97;
pub const VK_FORMAT_R32_UINT: u32 = 98;
pub const VK_FORMAT_R32_SINT: u32 = 99;
pub const VK_FORMAT_R32_SFLOAT: u32 = 100;
pub const VK_FORMAT_R32G32_UINT: u32 = 101;
pub const VK_FORMAT_R32G32_SINT: u32 = 102;
pub const VK_FORMAT_R32G32_SFLOAT: u32 = 103;
pub const VK_FORMAT_R32G32B32_UINT: u32 = 104;
pub const VK_FORMAT_R32G32B32_SINT: u32 = 105;
pub const VK_FORMAT_R32G32B32_SFLOAT: u32 = 106;
pub const VK_FORMAT_R32G32B32A32_UINT: u32 = 107;
pub const VK_FORMAT_R32G32B32A32_SINT: u32 = 108;
pub const VK_FORMAT_R32G32B32A32_SFLOAT: u32 = 109;
pub const VK_FORMAT_R64_UINT: u32 = 110;
pub const VK_FORMAT_R64_SINT: u32 = 111;
pub const VK_FORMAT_R64_SFLOAT: u32 = 112;
pub const VK_FORMAT_R64G64_UINT: u32 = 113;
pub const VK_FORMAT_R64G64_SINT: u32 = 114;
pub const VK_FORMAT_R64G64_SFLOAT: u32 = 115;
pub const VK_FORMAT_R64G64B64_UINT: u32 = 116;
pub const VK_FORMAT_R64G64B64_SINT: u32 = 117;
pub const VK_FORMAT_R64G64B64_SFLOAT: u32 = 118;
pub const VK_FORMAT_R64G64B64A64_UINT: u32 = 119;
pub const VK_FORMAT_R64G64B64A64_SINT: u32 = 120;
pub const VK_FORMAT_R64G64B64A64_SFLOAT: u32 = 121;
pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: u32 = 122;
pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: u32 = 123;
pub const VK_FORMAT_D16_UNORM: u32 = 124;
pub const VK_FORMAT_X8_D24_UNORM_PACK32: u32 = 125;
pub const VK_FORMAT_D32_SFLOAT: u32 = 126;
pub const VK_FORMAT_S8_UINT: u32 = 127;
pub const VK_FORMAT_D16_UNORM_S8_UINT: u32 = 128;
pub const VK_FORMAT_D24_UNORM_S8_UINT: u32 = 129;
pub const VK_FORMAT_D32_SFLOAT_S8_UINT: u32 = 130;
pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: u32 = 131;
pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: u32 = 132;
pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: u32 = 133;
pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: u32 = 134;
pub const VK_FORMAT_BC2_UNORM_BLOCK: u32 = 135;
pub const VK_FORMAT_BC2_SRGB_BLOCK: u32 = 136;
pub const VK_FORMAT_BC3_UNORM_BLOCK: u32 = 137;
pub const VK_FORMAT_BC3_SRGB_BLOCK: u32 = 138;
pub const VK_FORMAT_BC4_UNORM_BLOCK: u32 = 139;
pub const VK_FORMAT_BC4_SNORM_BLOCK: u32 = 140;
pub const VK_FORMAT_BC5_UNORM_BLOCK: u32 = 141;
pub const VK_FORMAT_BC5_SNORM_BLOCK: u32 = 142;
pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: u32 = 143;
pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: u32 = 144;
pub const VK_FORMAT_BC7_UNORM_BLOCK: u32 = 145;
pub const VK_FORMAT_BC7_SRGB_BLOCK: u32 = 146;
pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: u32 = 147;
pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: u32 = 148;
pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: u32 = 149;
pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: u32 = 150;
pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: u32 = 151;
pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: u32 = 152;
pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: u32 = 153;
pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: u32 = 154;
pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: u32 = 155;
pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: u32 = 156;
pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: u32 = 157;
pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: u32 = 158;
pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: u32 = 159;
pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: u32 = 160;
pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: u32 = 161;
pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: u32 = 162;
pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: u32 = 163;
pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: u32 = 164;
pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: u32 = 165;
pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: u32 = 166;
pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: u32 = 167;
pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: u32 = 168;
pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: u32 = 169;
pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: u32 = 170;
pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: u32 = 171;
pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: u32 = 172;
pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: u32 = 173;
pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: u32 = 174;
pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: u32 = 175;
pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: u32 = 176;
pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: u32 = 177;
pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: u32 = 178;
pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: u32 = 179;
pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: u32 = 180;
pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: u32 = 181;
pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: u32 = 182;
pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: u32 = 183;
pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: u32 = 184;
pub const VK_FORMAT_G8B8G8R8_422_UNORM: u32 = 1000156000;
pub const VK_FORMAT_B8G8R8G8_422_UNORM: u32 = 1000156001;
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM: u32 = 1000156002;
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM: u32 = 1000156003;
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM: u32 = 1000156004;
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM: u32 = 1000156005;
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM: u32 = 1000156006;
pub const VK_FORMAT_R10X6_UNORM_PACK16: u32 = 1000156007;
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16: u32 = 1000156008;
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: u32 = 1000156009;
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: u32 = 1000156010;
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: u32 = 1000156011;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: u32 = 1000156012;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: u32 = 1000156013;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: u32 = 1000156014;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: u32 = 1000156015;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: u32 = 1000156016;
pub const VK_FORMAT_R12X4_UNORM_PACK16: u32 = 1000156017;
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16: u32 = 1000156018;
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: u32 = 1000156019;
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: u32 = 1000156020;
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: u32 = 1000156021;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: u32 = 1000156022;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: u32 = 1000156023;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: u32 = 1000156024;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: u32 = 1000156025;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: u32 = 1000156026;
pub const VK_FORMAT_G16B16G16R16_422_UNORM: u32 = 1000156027;
pub const VK_FORMAT_B16G16R16G16_422_UNORM: u32 = 1000156028;
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM: u32 = 1000156029;
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM: u32 = 1000156030;
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM: u32 = 1000156031;
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM: u32 = 1000156032;
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM: u32 = 1000156033;
pub const VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG: u32 = 1000054000;
pub const VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG: u32 = 1000054001;
pub const VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG: u32 = 1000054002;
pub const VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG: u32 = 1000054003;
pub const VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG: u32 = 1000054004;
pub const VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG: u32 = 1000054005;
pub const VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG: u32 = 1000054006;
pub const VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG: u32 = 1000054007;
pub const VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK_EXT: u32 = 1000066000;
pub const VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK_EXT: u32 = 1000066001;
pub const VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK_EXT: u32 = 1000066002;
pub const VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK_EXT: u32 = 1000066003;
pub const VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK_EXT: u32 = 1000066004;
pub const VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK_EXT: u32 = 1000066005;
pub const VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK_EXT: u32 = 1000066006;
pub const VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK_EXT: u32 = 1000066007;
pub const VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK_EXT: u32 = 1000066008;
pub const VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK_EXT: u32 = 1000066009;
pub const VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK_EXT: u32 = 1000066010;
pub const VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK_EXT: u32 = 1000066011;
pub const VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK_EXT: u32 = 1000066012;
pub const VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK_EXT: u32 = 1000066013;
pub const VK_FORMAT_G8_B8R8_2PLANE_444_UNORM_EXT: u32 = 1000330000;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: u32 = 1000330001;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: u32 = 1000330002;
pub const VK_FORMAT_G16_B16R16_2PLANE_444_UNORM_EXT: u32 = 1000330003;
pub const VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT: u32 = 1000340000;
pub const VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT: u32 = 1000340001;
pub const VK_FORMAT_G8B8G8R8_422_UNORM_KHR: u32 = VK_FORMAT_G8B8G8R8_422_UNORM;
pub const VK_FORMAT_B8G8R8G8_422_UNORM_KHR: u32 = VK_FORMAT_B8G8R8G8_422_UNORM;
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR: u32 = VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM;
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR: u32 = VK_FORMAT_G8_B8R8_2PLANE_420_UNORM;
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR: u32 = VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM;
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR: u32 = VK_FORMAT_G8_B8R8_2PLANE_422_UNORM;
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR: u32 = VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM;
pub const VK_FORMAT_R10X6_UNORM_PACK16_KHR: u32 = VK_FORMAT_R10X6_UNORM_PACK16;
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR: u32 = VK_FORMAT_R10X6G10X6_UNORM_2PACK16;
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: u32 = VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16;
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: u32 = VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16;
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: u32 = VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: u32 =
    VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: u32 =
    VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: u32 =
    VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: u32 =
    VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: u32 =
    VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16;
pub const VK_FORMAT_R12X4_UNORM_PACK16_KHR: u32 = VK_FORMAT_R12X4_UNORM_PACK16;
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR: u32 = VK_FORMAT_R12X4G12X4_UNORM_2PACK16;
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: u32 = VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16;
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: u32 = VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16;
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: u32 = VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: u32 =
    VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: u32 =
    VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: u32 =
    VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: u32 =
    VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: u32 =
    VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16;
pub const VK_FORMAT_G16B16G16R16_422_UNORM_KHR: u32 = VK_FORMAT_G16B16G16R16_422_UNORM;
pub const VK_FORMAT_B16G16R16G16_422_UNORM_KHR: u32 = VK_FORMAT_B16G16R16G16_422_UNORM;
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR: u32 = VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM;
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR: u32 = VK_FORMAT_G16_B16R16_2PLANE_420_UNORM;
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR: u32 = VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM;
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR: u32 = VK_FORMAT_G16_B16R16_2PLANE_422_UNORM;
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR: u32 = VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM;

pub fn dxgi_to_vk(dxgi_format: u32) -> u32 {
    use scratch_dds::dxgi_format::*;

    match dxgi_format {
        DXGI_FORMAT_UNKNOWN => panic!("Cannot translate FORMAT_UNKNOWN"),
        DXGI_FORMAT_R32G32B32A32_FLOAT => VK_FORMAT_R32G32B32A32_SFLOAT,
        DXGI_FORMAT_R32G32B32A32_UINT => VK_FORMAT_R32G32B32A32_UINT,
        DXGI_FORMAT_R32G32B32A32_SINT => VK_FORMAT_R32G32B32A32_SINT,
        DXGI_FORMAT_R32G32B32_FLOAT => VK_FORMAT_R32G32B32_SFLOAT,
        DXGI_FORMAT_R32G32B32_UINT => VK_FORMAT_R32G32B32_UINT,
        DXGI_FORMAT_R32G32B32_SINT => VK_FORMAT_R32G32B32_SINT,
        DXGI_FORMAT_R16G16B16A16_FLOAT => VK_FORMAT_R16G16B16A16_SFLOAT,
        DXGI_FORMAT_R16G16B16A16_UNORM => VK_FORMAT_R16G16B16A16_UNORM,
        DXGI_FORMAT_R16G16B16A16_UINT => VK_FORMAT_R16G16B16A16_UINT,
        DXGI_FORMAT_R16G16B16A16_SNORM => VK_FORMAT_R16G16B16A16_SNORM,
        DXGI_FORMAT_R16G16B16A16_SINT => VK_FORMAT_R16G16B16A16_SINT,
        DXGI_FORMAT_R32G32_FLOAT => VK_FORMAT_R32G32_SFLOAT,
        DXGI_FORMAT_R32G32_UINT => VK_FORMAT_R32G32_UINT,
        DXGI_FORMAT_R32G32_SINT => VK_FORMAT_R32G32_SINT,
        DXGI_FORMAT_D32_FLOAT_S8X24_UINT => VK_FORMAT_X8_D24_UNORM_PACK32,
        DXGI_FORMAT_R10G10B10A2_UNORM => VK_FORMAT_A2R10G10B10_UNORM_PACK32,
        DXGI_FORMAT_R10G10B10A2_UINT => VK_FORMAT_A2R10G10B10_UINT_PACK32,
        DXGI_FORMAT_R11G11B10_FLOAT => VK_FORMAT_B10G11R11_UFLOAT_PACK32,
        DXGI_FORMAT_R8G8B8A8_UNORM => VK_FORMAT_R8G8B8A8_UNORM,
        DXGI_FORMAT_R8G8B8A8_UNORM_SRGB => VK_FORMAT_R8G8B8A8_SRGB,
        DXGI_FORMAT_R8G8B8A8_UINT => VK_FORMAT_R8G8B8A8_UINT,
        DXGI_FORMAT_R8G8B8A8_SNORM => VK_FORMAT_R8G8B8A8_SNORM,
        DXGI_FORMAT_R8G8B8A8_SINT => VK_FORMAT_R8G8B8A8_SINT,
        DXGI_FORMAT_R16G16_FLOAT => VK_FORMAT_R16G16_SFLOAT,
        DXGI_FORMAT_R16G16_UNORM => VK_FORMAT_R16G16_UNORM,
        DXGI_FORMAT_R16G16_UINT => VK_FORMAT_R16G16_UINT,
        DXGI_FORMAT_R16G16_SNORM => VK_FORMAT_R16G16_SNORM,
        DXGI_FORMAT_R16G16_SINT => VK_FORMAT_R16G16_SINT,
        DXGI_FORMAT_D32_FLOAT => VK_FORMAT_D32_SFLOAT,
        DXGI_FORMAT_R32_FLOAT => VK_FORMAT_R32_SFLOAT,
        DXGI_FORMAT_R32_UINT => VK_FORMAT_R32_UINT,
        DXGI_FORMAT_R32_SINT => VK_FORMAT_R32_SINT,
        DXGI_FORMAT_D24_UNORM_S8_UINT => VK_FORMAT_D24_UNORM_S8_UINT,
        DXGI_FORMAT_R8G8_UNORM => VK_FORMAT_R8G8_UNORM,
        DXGI_FORMAT_R8G8_UINT => VK_FORMAT_R8G8_UINT,
        DXGI_FORMAT_R8G8_SNORM => VK_FORMAT_R8G8_SNORM,
        DXGI_FORMAT_R8G8_SINT => VK_FORMAT_R8G8_SINT,
        DXGI_FORMAT_R16_FLOAT => VK_FORMAT_R16_SFLOAT,
        DXGI_FORMAT_D16_UNORM => VK_FORMAT_D16_UNORM,
        DXGI_FORMAT_R16_UNORM => VK_FORMAT_R16_UNORM,
        DXGI_FORMAT_R16_UINT => VK_FORMAT_R16_UINT,
        DXGI_FORMAT_R16_SNORM => VK_FORMAT_R16_SNORM,
        DXGI_FORMAT_R16_SINT => VK_FORMAT_R16_SINT,
        DXGI_FORMAT_R8_UNORM => VK_FORMAT_R8_UNORM,
        DXGI_FORMAT_R8_UINT => VK_FORMAT_R8_UINT,
        DXGI_FORMAT_R8_SNORM => VK_FORMAT_R8_SNORM,
        DXGI_FORMAT_R8_SINT => VK_FORMAT_R8_SINT,
        DXGI_FORMAT_A8_UNORM => VK_FORMAT_R8_UNORM,
        DXGI_FORMAT_R9G9B9E5_SHAREDEXP => VK_FORMAT_E5B9G9R9_UFLOAT_PACK32,
        DXGI_FORMAT_BC2_UNORM => VK_FORMAT_BC2_UNORM_BLOCK,
        DXGI_FORMAT_BC2_UNORM_SRGB => VK_FORMAT_BC2_SRGB_BLOCK,
        DXGI_FORMAT_BC3_UNORM => VK_FORMAT_BC3_UNORM_BLOCK,
        DXGI_FORMAT_BC3_UNORM_SRGB => VK_FORMAT_BC3_SRGB_BLOCK,
        DXGI_FORMAT_BC4_UNORM => VK_FORMAT_BC4_UNORM_BLOCK,
        DXGI_FORMAT_BC4_SNORM => VK_FORMAT_BC4_SNORM_BLOCK,
        DXGI_FORMAT_BC5_UNORM => VK_FORMAT_BC5_UNORM_BLOCK,
        DXGI_FORMAT_BC5_SNORM => VK_FORMAT_BC5_SNORM_BLOCK,
        DXGI_FORMAT_B5G6R5_UNORM => VK_FORMAT_B5G6R5_UNORM_PACK16,
        DXGI_FORMAT_B5G5R5A1_UNORM => VK_FORMAT_B5G5R5A1_UNORM_PACK16,
        DXGI_FORMAT_B8G8R8A8_UNORM => VK_FORMAT_B8G8R8A8_UNORM,
        DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM => VK_FORMAT_A2R10G10B10_UNORM_PACK32,
        DXGI_FORMAT_B8G8R8A8_UNORM_SRGB => VK_FORMAT_B8G8R8A8_SRGB,
        DXGI_FORMAT_BC6H_UF16 => VK_FORMAT_BC6H_UFLOAT_BLOCK,
        DXGI_FORMAT_BC6H_SF16 => VK_FORMAT_BC6H_SFLOAT_BLOCK,
        DXGI_FORMAT_BC7_UNORM => VK_FORMAT_BC7_UNORM_BLOCK,
        DXGI_FORMAT_BC7_UNORM_SRGB => VK_FORMAT_BC7_SRGB_BLOCK,

        DXGI_FORMAT_AYUV => panic!("Cannot translate AYUV"),
        DXGI_FORMAT_Y410 => panic!("Cannot translate Y410"),
        DXGI_FORMAT_Y416 => panic!("Cannot translate Y416"),
        DXGI_FORMAT_NV12 => panic!("Cannot translate NV12"),
        DXGI_FORMAT_P010 => panic!("Cannot translate P010"),
        DXGI_FORMAT_P016 => panic!("Cannot translate P016"),
        DXGI_FORMAT_420_OPAQUE => panic!("Cannot translate 420_OPAQUE"),
        DXGI_FORMAT_YUY2 => panic!("Cannot translate YUY2"),
        DXGI_FORMAT_Y210 => panic!("Cannot translate Y210"),
        DXGI_FORMAT_Y216 => panic!("Cannot translate Y216"),
        DXGI_FORMAT_NV11 => panic!("Cannot translate NV11"),
        DXGI_FORMAT_AI44 => panic!("Cannot translate AI44"),
        DXGI_FORMAT_IA44 => panic!("Cannot translate IA44"),
        DXGI_FORMAT_P8 => panic!("Cannot translate P8"),
        DXGI_FORMAT_A8P8 => panic!("Cannot translate A8P8"),
        DXGI_FORMAT_B4G4R4A4_UNORM => panic!("Cannot translate B4G4R4A4_UNORM"),
        DXGI_FORMAT_P208 => panic!("Cannot translate P208"),
        DXGI_FORMAT_V208 => panic!("Cannot translate V208"),
        DXGI_FORMAT_V408 => panic!("Cannot translate V408"),

        DXGI_FORMAT_B8G8R8X8_UNORM_SRGB => panic!("Cannot translate B8G8R8X8_UNORM_SRGB"),

        DXGI_FORMAT_B8G8R8X8_UNORM => panic!("Cannot translate B8G8R8X8_UNORM"),

        DXGI_FORMAT_BC1_UNORM => panic!("BC1 requires context about alpha channel, not implemented yet"),
        DXGI_FORMAT_BC1_UNORM_SRGB => panic!("BC1 requires context about alpha channel, not implemented yet"),

        DXGI_FORMAT_R8G8_B8G8_UNORM => panic!("Cannot translate R8G8_B8G8_UNORM"),
        DXGI_FORMAT_G8R8_G8B8_UNORM => panic!("Cannot translate G8R8_G8B8_UNORM"),

        DXGI_FORMAT_R1_UNORM => panic!("Cannot translate R1_UNORM format"),

        DXGI_FORMAT_R32G32B32A32_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R32G32B32_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R16G16B16A16_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R32G32_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R32G8X24_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_X32_TYPELESS_G8X24_UINT => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R10G10B10A2_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R8G8B8A8_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R16G16_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R32_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R24G8_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R24_UNORM_X8_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_X24_TYPELESS_G8_UINT => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R8G8_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R16_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_R8_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_BC1_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_BC2_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_BC3_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_BC4_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_BC5_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_B8G8R8A8_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_B8G8R8X8_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_BC6H_TYPELESS => panic!("Cannot translate typeless format"),
        DXGI_FORMAT_BC7_TYPELESS => panic!("Cannot translate typeless format"),

        _ => panic!("Unknown dxgi format: {}", dxgi_format),
    }
}
