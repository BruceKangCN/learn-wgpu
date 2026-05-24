use std::path::Path;

use anyhow::Result;
use wgpu::util::align_to;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Texture {
    #[allow(unused)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: impl AsRef<Path>,
        label: Option<&str>,
    ) -> Result<Self> {
        let img = image::open(path)?.to_rgba8();
        let dims = img.dimensions();

        let size = wgpu::Extent3d {
            width: dims.0,
            height: dims.1,

            // All textures are stored as 3D. We represent our 2D texture by
            // setting depth to 1.
            depth_or_array_layers: 1,
        };

        let tex_desc = wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,

            // - TEXTURE_BINDING: we want to use this texture in shaders
            // COPY_DST: we want to copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,

            view_formats: &[],
        };
        let texture = device.create_texture(&tex_desc);

        let copy_texture_info = wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        };
        assert_eq!(4 * dims.0, align_to(4 * dims.0, 256));
        let copy_buffer_layout = wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * dims.0),
            rows_per_image: Some(dims.1),
        };
        queue.write_texture(copy_texture_info, &img, copy_buffer_layout, size);

        let view = texture.create_view(&Default::default());
        let sampler_desc = wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        };
        let sampler = device.create_sampler(&sampler_desc);

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}
