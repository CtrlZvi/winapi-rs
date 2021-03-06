// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate winapi;
use std::mem::{size_of, align_of};
use winapi::*;
#[test] #[cfg(target_arch = "x86")]
fn template() {
}
#[test] #[cfg(target_arch = "x86_64")]
fn template() {
}
#[test] #[cfg(target_arch = "x86")]
fn d3d11() {
    assert_eq!(size_of::<D3D11_INPUT_ELEMENT_DESC>(), 28);
    assert_eq!(align_of::<D3D11_INPUT_ELEMENT_DESC>(), 4);
    assert_eq!(size_of::<D3D11_SO_DECLARATION_ENTRY>(), 16);
    assert_eq!(align_of::<D3D11_SO_DECLARATION_ENTRY>(), 4);
    assert_eq!(size_of::<D3D11_VIEWPORT>(), 24);
    assert_eq!(align_of::<D3D11_VIEWPORT>(), 4);
    assert_eq!(size_of::<D3D11_DRAW_INSTANCED_INDIRECT_ARGS>(), 16);
    assert_eq!(align_of::<D3D11_DRAW_INSTANCED_INDIRECT_ARGS>(), 4);
    assert_eq!(size_of::<D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS>(), 20);
    assert_eq!(align_of::<D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS>(), 4);
    assert_eq!(size_of::<D3D11_BOX>(), 24);
    assert_eq!(align_of::<D3D11_BOX>(), 4);
    assert_eq!(size_of::<D3D11_DEPTH_STENCILOP_DESC>(), 16);
    assert_eq!(align_of::<D3D11_DEPTH_STENCILOP_DESC>(), 4);
    assert_eq!(size_of::<D3D11_DEPTH_STENCIL_DESC>(), 52);
    assert_eq!(align_of::<D3D11_DEPTH_STENCIL_DESC>(), 4);
    assert_eq!(size_of::<D3D11_RENDER_TARGET_BLEND_DESC>(), 32);
    assert_eq!(align_of::<D3D11_RENDER_TARGET_BLEND_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BLEND_DESC>(), 264);
    assert_eq!(align_of::<D3D11_BLEND_DESC>(), 4);
    assert_eq!(size_of::<D3D11_RASTERIZER_DESC>(), 40);
    assert_eq!(align_of::<D3D11_RASTERIZER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_SUBRESOURCE_DATA>(), 12);
    assert_eq!(align_of::<D3D11_SUBRESOURCE_DATA>(), 4);
    assert_eq!(size_of::<D3D11_MAPPED_SUBRESOURCE>(), 12);
    assert_eq!(align_of::<D3D11_MAPPED_SUBRESOURCE>(), 4);
    assert_eq!(size_of::<D3D11_BUFFER_DESC>(), 24);
    assert_eq!(align_of::<D3D11_BUFFER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEXTURE1D_DESC>(), 32);
    assert_eq!(align_of::<D3D11_TEXTURE1D_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEXTURE2D_DESC>(), 44);
    assert_eq!(align_of::<D3D11_TEXTURE2D_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEXTURE3D_DESC>(), 36);
    assert_eq!(align_of::<D3D11_TEXTURE3D_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BUFFER_SRV>(), 8);
    assert_eq!(align_of::<D3D11_BUFFER_SRV>(), 4);
    assert_eq!(size_of::<D3D11_BUFFEREX_SRV>(), 12);
    assert_eq!(align_of::<D3D11_BUFFEREX_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX1D_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_SRV>(), 16);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2D_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_SRV>(), 16);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX3D_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX3D_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEXCUBE_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEXCUBE_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEXCUBE_ARRAY_SRV>(), 16);
    assert_eq!(align_of::<D3D11_TEXCUBE_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_SRV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2DMS_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_ARRAY_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2DMS_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_SHADER_RESOURCE_VIEW_DESC>(), 24);
    assert_eq!(align_of::<D3D11_SHADER_RESOURCE_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BUFFER_RTV>(), 8);
    assert_eq!(align_of::<D3D11_BUFFER_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_RTV>(), 4);
    assert_eq!(align_of::<D3D11_TEX1D_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_RTV>(), 12);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_RTV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_RTV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2DMS_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_RTV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_ARRAY_RTV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2DMS_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX3D_RTV>(), 12);
    assert_eq!(align_of::<D3D11_TEX3D_RTV>(), 4);
    assert_eq!(size_of::<D3D11_RENDER_TARGET_VIEW_DESC>(), 20);
    assert_eq!(align_of::<D3D11_RENDER_TARGET_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_DSV>(), 4);
    assert_eq!(align_of::<D3D11_TEX1D_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_DSV>(), 12);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_DSV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_DSV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_DSV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2DMS_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_ARRAY_DSV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2DMS_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D11_DEPTH_STENCIL_VIEW_DESC>(), 24);
    assert_eq!(align_of::<D3D11_DEPTH_STENCIL_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BUFFER_UAV>(), 12);
    assert_eq!(align_of::<D3D11_BUFFER_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_UAV>(), 4);
    assert_eq!(align_of::<D3D11_TEX1D_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_UAV>(), 12);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_UAV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_UAV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX3D_UAV>(), 12);
    assert_eq!(align_of::<D3D11_TEX3D_UAV>(), 4);
    assert_eq!(size_of::<D3D11_UNORDERED_ACCESS_VIEW_DESC>(), 20);
    assert_eq!(align_of::<D3D11_UNORDERED_ACCESS_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_SAMPLER_DESC>(), 52);
    assert_eq!(align_of::<D3D11_SAMPLER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_QUERY_DESC>(), 8);
    assert_eq!(align_of::<D3D11_QUERY_DESC>(), 4);
    assert_eq!(size_of::<D3D11_QUERY_DATA_TIMESTAMP_DISJOINT>(), 16);
    assert_eq!(align_of::<D3D11_QUERY_DATA_TIMESTAMP_DISJOINT>(), 8);
    assert_eq!(size_of::<D3D11_QUERY_DATA_PIPELINE_STATISTICS>(), 88);
    assert_eq!(align_of::<D3D11_QUERY_DATA_PIPELINE_STATISTICS>(), 8);
    assert_eq!(size_of::<D3D11_QUERY_DATA_SO_STATISTICS>(), 16);
    assert_eq!(align_of::<D3D11_QUERY_DATA_SO_STATISTICS>(), 8);
    assert_eq!(size_of::<D3D11_COUNTER_DESC>(), 8);
    assert_eq!(align_of::<D3D11_COUNTER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_COUNTER_INFO>(), 12);
    assert_eq!(align_of::<D3D11_COUNTER_INFO>(), 4);
    assert_eq!(size_of::<D3D11_CLASS_INSTANCE_DESC>(), 32);
    assert_eq!(align_of::<D3D11_CLASS_INSTANCE_DESC>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_THREADING>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_THREADING>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_DOUBLES>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_DOUBLES>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT2>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT2>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS>(), 56);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_ARCHITECTURE_INFO>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_ARCHITECTURE_INFO>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS1>(), 16);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS1>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_MARKER_SUPPORT>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_MARKER_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS1>(), 16);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS1>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS2>(), 32);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS2>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS3>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS3>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_DESC>(), 28);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_CONFIG>(), 100);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_CONFIG>(), 4);
    assert_eq!(size_of::<D3D11_AES_CTR_IV>(), 16);
    assert_eq!(align_of::<D3D11_AES_CTR_IV>(), 8);
    assert_eq!(size_of::<D3D11_ENCRYPTED_BLOCK_INFO>(), 12);
    assert_eq!(align_of::<D3D11_ENCRYPTED_BLOCK_INFO>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_BUFFER_DESC>(), 64);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_BUFFER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_EXTENSION>(), 28);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_EXTENSION>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_CAPS>(), 36);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_CAPS>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS>(), 20);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_CONTENT_PROTECTION_CAPS>(), 24);
    assert_eq!(align_of::<D3D11_VIDEO_CONTENT_PROTECTION_CAPS>(), 8);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_CUSTOM_RATE>(), 20);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_CUSTOM_RATE>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_CONTENT_DESC>(), 40);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_CONTENT_DESC>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_COLOR_RGBA>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_COLOR_RGBA>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_COLOR_YCbCrA>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_COLOR_YCbCrA>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_COLOR>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_COLOR>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_COLOR_SPACE>(), 4);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_COLOR_SPACE>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_STREAM>(), 44);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_STREAM>(), 4);
    assert_eq!(size_of::<D3D11_OMAC>(), 16);
    assert_eq!(align_of::<D3D11_OMAC>(), 1);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_INPUT>(), 24);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT>(), 44);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_PROTECTION_FLAGS>(), 4);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_PROTECTION_FLAGS>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT>(), 28);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT>(), 28);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT>(), 32);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT>(), 36);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT>(), 64);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_ACESSIBILITY_OUTPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_ACESSIBILITY_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT>(), 28);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT>(), 64);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT>(), 60);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_INPUT>(), 40);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_OUTPUT>(), 44);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_OUTPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT>(), 44);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT>(), 52);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT>(), 52);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_VDOV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_VDOV>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC>(), 24);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_VPIV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2D_VPIV>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_VPOV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_VPOV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_VPOV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_VPOV>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC>(), 4);
}
#[test] #[cfg(target_arch = "x86_64")]
fn d3d11() {
    assert_eq!(size_of::<D3D11_INPUT_ELEMENT_DESC>(), 32);
    assert_eq!(align_of::<D3D11_INPUT_ELEMENT_DESC>(), 8);
    assert_eq!(size_of::<D3D11_SO_DECLARATION_ENTRY>(), 24);
    assert_eq!(align_of::<D3D11_SO_DECLARATION_ENTRY>(), 8);
    assert_eq!(size_of::<D3D11_VIEWPORT>(), 24);
    assert_eq!(align_of::<D3D11_VIEWPORT>(), 4);
    assert_eq!(size_of::<D3D11_DRAW_INSTANCED_INDIRECT_ARGS>(), 16);
    assert_eq!(align_of::<D3D11_DRAW_INSTANCED_INDIRECT_ARGS>(), 4);
    assert_eq!(size_of::<D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS>(), 20);
    assert_eq!(align_of::<D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS>(), 4);
    assert_eq!(size_of::<D3D11_BOX>(), 24);
    assert_eq!(align_of::<D3D11_BOX>(), 4);
    assert_eq!(size_of::<D3D11_DEPTH_STENCILOP_DESC>(), 16);
    assert_eq!(align_of::<D3D11_DEPTH_STENCILOP_DESC>(), 4);
    assert_eq!(size_of::<D3D11_DEPTH_STENCIL_DESC>(), 52);
    assert_eq!(align_of::<D3D11_DEPTH_STENCIL_DESC>(), 4);
    assert_eq!(size_of::<D3D11_RENDER_TARGET_BLEND_DESC>(), 32);
    assert_eq!(align_of::<D3D11_RENDER_TARGET_BLEND_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BLEND_DESC>(), 264);
    assert_eq!(align_of::<D3D11_BLEND_DESC>(), 4);
    assert_eq!(size_of::<D3D11_RASTERIZER_DESC>(), 40);
    assert_eq!(align_of::<D3D11_RASTERIZER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_SUBRESOURCE_DATA>(), 16);
    assert_eq!(align_of::<D3D11_SUBRESOURCE_DATA>(), 8);
    assert_eq!(size_of::<D3D11_MAPPED_SUBRESOURCE>(), 16);
    assert_eq!(align_of::<D3D11_MAPPED_SUBRESOURCE>(), 8);
    assert_eq!(size_of::<D3D11_BUFFER_DESC>(), 24);
    assert_eq!(align_of::<D3D11_BUFFER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEXTURE1D_DESC>(), 32);
    assert_eq!(align_of::<D3D11_TEXTURE1D_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEXTURE2D_DESC>(), 44);
    assert_eq!(align_of::<D3D11_TEXTURE2D_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEXTURE3D_DESC>(), 36);
    assert_eq!(align_of::<D3D11_TEXTURE3D_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BUFFER_SRV>(), 8);
    assert_eq!(align_of::<D3D11_BUFFER_SRV>(), 4);
    assert_eq!(size_of::<D3D11_BUFFEREX_SRV>(), 12);
    assert_eq!(align_of::<D3D11_BUFFEREX_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX1D_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_SRV>(), 16);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2D_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_SRV>(), 16);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX3D_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX3D_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEXCUBE_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEXCUBE_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEXCUBE_ARRAY_SRV>(), 16);
    assert_eq!(align_of::<D3D11_TEXCUBE_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_SRV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2DMS_SRV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_ARRAY_SRV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2DMS_ARRAY_SRV>(), 4);
    assert_eq!(size_of::<D3D11_SHADER_RESOURCE_VIEW_DESC>(), 24);
    assert_eq!(align_of::<D3D11_SHADER_RESOURCE_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BUFFER_RTV>(), 8);
    assert_eq!(align_of::<D3D11_BUFFER_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_RTV>(), 4);
    assert_eq!(align_of::<D3D11_TEX1D_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_RTV>(), 12);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_RTV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_RTV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2DMS_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_RTV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_ARRAY_RTV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2DMS_ARRAY_RTV>(), 4);
    assert_eq!(size_of::<D3D11_TEX3D_RTV>(), 12);
    assert_eq!(align_of::<D3D11_TEX3D_RTV>(), 4);
    assert_eq!(size_of::<D3D11_RENDER_TARGET_VIEW_DESC>(), 20);
    assert_eq!(align_of::<D3D11_RENDER_TARGET_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_DSV>(), 4);
    assert_eq!(align_of::<D3D11_TEX1D_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_DSV>(), 12);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_DSV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_DSV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_DSV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2DMS_DSV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2DMS_ARRAY_DSV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2DMS_ARRAY_DSV>(), 4);
    assert_eq!(size_of::<D3D11_DEPTH_STENCIL_VIEW_DESC>(), 24);
    assert_eq!(align_of::<D3D11_DEPTH_STENCIL_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_BUFFER_UAV>(), 12);
    assert_eq!(align_of::<D3D11_BUFFER_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_UAV>(), 4);
    assert_eq!(align_of::<D3D11_TEX1D_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX1D_ARRAY_UAV>(), 12);
    assert_eq!(align_of::<D3D11_TEX1D_ARRAY_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_UAV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_UAV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_UAV>(), 4);
    assert_eq!(size_of::<D3D11_TEX3D_UAV>(), 12);
    assert_eq!(align_of::<D3D11_TEX3D_UAV>(), 4);
    assert_eq!(size_of::<D3D11_UNORDERED_ACCESS_VIEW_DESC>(), 20);
    assert_eq!(align_of::<D3D11_UNORDERED_ACCESS_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_SAMPLER_DESC>(), 52);
    assert_eq!(align_of::<D3D11_SAMPLER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_QUERY_DESC>(), 8);
    assert_eq!(align_of::<D3D11_QUERY_DESC>(), 4);
    assert_eq!(size_of::<D3D11_QUERY_DATA_TIMESTAMP_DISJOINT>(), 16);
    assert_eq!(align_of::<D3D11_QUERY_DATA_TIMESTAMP_DISJOINT>(), 8);
    assert_eq!(size_of::<D3D11_QUERY_DATA_PIPELINE_STATISTICS>(), 88);
    assert_eq!(align_of::<D3D11_QUERY_DATA_PIPELINE_STATISTICS>(), 8);
    assert_eq!(size_of::<D3D11_QUERY_DATA_SO_STATISTICS>(), 16);
    assert_eq!(align_of::<D3D11_QUERY_DATA_SO_STATISTICS>(), 8);
    assert_eq!(size_of::<D3D11_COUNTER_DESC>(), 8);
    assert_eq!(align_of::<D3D11_COUNTER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_COUNTER_INFO>(), 12);
    assert_eq!(align_of::<D3D11_COUNTER_INFO>(), 4);
    assert_eq!(size_of::<D3D11_CLASS_INSTANCE_DESC>(), 32);
    assert_eq!(align_of::<D3D11_CLASS_INSTANCE_DESC>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_THREADING>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_THREADING>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_DOUBLES>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_DOUBLES>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT2>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_FORMAT_SUPPORT2>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS>(), 56);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_ARCHITECTURE_INFO>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_ARCHITECTURE_INFO>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS1>(), 16);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS1>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_MARKER_SUPPORT>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_MARKER_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS1>(), 16);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D9_OPTIONS1>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS2>(), 32);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS2>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS3>(), 4);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS3>(), 4);
    assert_eq!(size_of::<D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT>(), 8);
    assert_eq!(align_of::<D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_DESC>(), 28);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_DESC>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_CONFIG>(), 100);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_CONFIG>(), 4);
    assert_eq!(size_of::<D3D11_AES_CTR_IV>(), 16);
    assert_eq!(align_of::<D3D11_AES_CTR_IV>(), 8);
    assert_eq!(size_of::<D3D11_ENCRYPTED_BLOCK_INFO>(), 12);
    assert_eq!(align_of::<D3D11_ENCRYPTED_BLOCK_INFO>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_BUFFER_DESC>(), 72);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_BUFFER_DESC>(), 8);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_EXTENSION>(), 48);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_EXTENSION>(), 8);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_CAPS>(), 36);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_CAPS>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS>(), 20);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_CONTENT_PROTECTION_CAPS>(), 24);
    assert_eq!(align_of::<D3D11_VIDEO_CONTENT_PROTECTION_CAPS>(), 8);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_CUSTOM_RATE>(), 20);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_CUSTOM_RATE>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_CONTENT_DESC>(), 40);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_CONTENT_DESC>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_COLOR_RGBA>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_COLOR_RGBA>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_COLOR_YCbCrA>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_COLOR_YCbCrA>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_COLOR>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_COLOR>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_COLOR_SPACE>(), 4);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_COLOR_SPACE>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_STREAM>(), 72);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_STREAM>(), 8);
    assert_eq!(size_of::<D3D11_OMAC>(), 16);
    assert_eq!(align_of::<D3D11_OMAC>(), 1);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_INPUT>(), 32);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_PROTECTION_FLAGS>(), 4);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_PROTECTION_FLAGS>(), 4);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT>(), 40);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT>(), 72);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT>(), 40);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT>(), 64);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT>(), 72);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT>(), 80);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_ACESSIBILITY_OUTPUT>(), 64);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_ACESSIBILITY_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT>(), 40);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT>(), 72);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT>(), 64);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_INPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_OUTPUT>(), 48);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_OUTPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT>(), 56);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT>(), 72);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT>(), 72);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT>(), 64);
    assert_eq!(align_of::<D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT>(), 8);
    assert_eq!(size_of::<D3D11_TEX2D_VDOV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_VDOV>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC>(), 24);
    assert_eq!(align_of::<D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_VPIV>(), 8);
    assert_eq!(align_of::<D3D11_TEX2D_VPIV>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_VPOV>(), 4);
    assert_eq!(align_of::<D3D11_TEX2D_VPOV>(), 4);
    assert_eq!(size_of::<D3D11_TEX2D_ARRAY_VPOV>(), 12);
    assert_eq!(align_of::<D3D11_TEX2D_ARRAY_VPOV>(), 4);
    assert_eq!(size_of::<D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC>(), 16);
    assert_eq!(align_of::<D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC>(), 4);
}
#[test] #[cfg(target_arch = "x86")]
fn hidclass() {
    assert_eq!(size_of::<HID_XFER_PACKET>(), 12);
    assert_eq!(align_of::<HID_XFER_PACKET>(), 4);
    assert_eq!(size_of::<HID_COLLECTION_INFORMATION>(), 12);
    assert_eq!(align_of::<HID_COLLECTION_INFORMATION>(), 4);
    assert_eq!(size_of::<HID_DRIVER_CONFIG>(), 8);
    assert_eq!(align_of::<HID_DRIVER_CONFIG>(), 4);
}
#[test] #[cfg(target_arch = "x86_64")]
fn hidclass() {
    assert_eq!(size_of::<HID_XFER_PACKET>(), 16);
    assert_eq!(align_of::<HID_XFER_PACKET>(), 8);
    assert_eq!(size_of::<HID_COLLECTION_INFORMATION>(), 12);
    assert_eq!(align_of::<HID_COLLECTION_INFORMATION>(), 4);
    assert_eq!(size_of::<HID_DRIVER_CONFIG>(), 8);
    assert_eq!(align_of::<HID_DRIVER_CONFIG>(), 4);
}
#[test] #[cfg(target_arch = "x86")]
fn hidpi() {
    assert_eq!(size_of::<USAGE_AND_PAGE>(), 4);
    assert_eq!(align_of::<USAGE_AND_PAGE>(), 2);
    assert_eq!(size_of::<HIDP_BUTTON_CAPS>(), 72);
    assert_eq!(align_of::<HIDP_BUTTON_CAPS>(), 4);
    assert_eq!(size_of::<HIDP_VALUE_CAPS>(), 72);
    assert_eq!(align_of::<HIDP_VALUE_CAPS>(), 4);
    assert_eq!(size_of::<HIDP_LINK_COLLECTION_NODE>(), 20);
    assert_eq!(align_of::<HIDP_LINK_COLLECTION_NODE>(), 4);
    assert_eq!(size_of::<HIDP_CAPS>(), 64);
    assert_eq!(align_of::<HIDP_CAPS>(), 2);
    assert_eq!(size_of::<HIDP_DATA>(), 8);
    assert_eq!(align_of::<HIDP_DATA>(), 4);
    assert_eq!(size_of::<HIDP_UNKNOWN_TOKEN>(), 8);
    assert_eq!(align_of::<HIDP_UNKNOWN_TOKEN>(), 4);
    assert_eq!(size_of::<HIDP_EXTENDED_ATTRIBUTES>(), 12);
    assert_eq!(align_of::<HIDP_EXTENDED_ATTRIBUTES>(), 4);
    assert_eq!(size_of::<HIDP_KEYBOARD_MODIFIER_STATE>(), 4);
    assert_eq!(align_of::<HIDP_KEYBOARD_MODIFIER_STATE>(), 4);
}
#[test] #[cfg(target_arch = "x86_64")]
fn hidpi() {
    assert_eq!(size_of::<USAGE_AND_PAGE>(), 4);
    assert_eq!(align_of::<USAGE_AND_PAGE>(), 2);
    assert_eq!(size_of::<HIDP_BUTTON_CAPS>(), 72);
    assert_eq!(align_of::<HIDP_BUTTON_CAPS>(), 4);
    assert_eq!(size_of::<HIDP_VALUE_CAPS>(), 72);
    assert_eq!(align_of::<HIDP_VALUE_CAPS>(), 4);
    assert_eq!(size_of::<HIDP_LINK_COLLECTION_NODE>(), 24);
    // assert_eq!(align_of::<HIDP_LINK_COLLECTION_NODE>(), 4); // FIXME
    assert_eq!(size_of::<HIDP_CAPS>(), 64);
    assert_eq!(align_of::<HIDP_CAPS>(), 2);
    assert_eq!(size_of::<HIDP_DATA>(), 8);
    assert_eq!(align_of::<HIDP_DATA>(), 4);
    assert_eq!(size_of::<HIDP_UNKNOWN_TOKEN>(), 8);
    assert_eq!(align_of::<HIDP_UNKNOWN_TOKEN>(), 4);
    // assert_eq!(size_of::<HIDP_EXTENDED_ATTRIBUTES>(), 16); // FIXME
    // assert_eq!(align_of::<HIDP_EXTENDED_ATTRIBUTES>(), 4); // FIXME
    assert_eq!(size_of::<HIDP_KEYBOARD_MODIFIER_STATE>(), 4);
    assert_eq!(align_of::<HIDP_KEYBOARD_MODIFIER_STATE>(), 4);
}
#[test] #[cfg(target_arch = "x86")]
fn hidsdi() {
    assert_eq!(size_of::<HIDD_CONFIGURATION>(), 12);
    assert_eq!(align_of::<HIDD_CONFIGURATION>(), 4);
    assert_eq!(size_of::<HIDD_ATTRIBUTES>(), 12);
    assert_eq!(align_of::<HIDD_ATTRIBUTES>(), 4);
}
#[test] #[cfg(target_arch = "x86_64")]
fn hidsdi() {
    assert_eq!(size_of::<HIDD_CONFIGURATION>(), 16);
    // assert_eq!(align_of::<HIDD_CONFIGURATION>(), 4); // FIXME
    assert_eq!(size_of::<HIDD_ATTRIBUTES>(), 12);
    assert_eq!(align_of::<HIDD_ATTRIBUTES>(), 4);
}
