use std::error::Error;
use std::path::Path;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float; // 1.

    pub fn load<P: AsRef<Path>>(
        device: &wgpu::Device,
        path: P,
    ) -> Result<(Self, wgpu::CommandBuffer), Box<dyn Error>> {
        let img = image::open(path)?;
        Self::from_image(device, img)
    }

    pub fn make_depth_texture(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Self {
        let desc = wgpu::TextureDescriptor {
            format: Self::DEPTH_FORMAT,

            // rendering to this texture
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,

            ..sc_desc.to_texture_desc()
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_default_view();
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            // 4.
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare_function: wgpu::CompareFunction::Always,
        });

        Self {
            texture,
            view,
            sampler,
        }
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        bytes: &[u8],
    ) -> Result<(Self, wgpu::CommandBuffer), Box<dyn Error>> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, img)
    }

    pub fn from_image(
        device: &wgpu::Device,
        img: image::DynamicImage,
    ) -> Result<(Self, wgpu::CommandBuffer), Box<dyn Error>> {
        let rgba = img.into_rgba();
        let dimensions = rgba.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,

            // only one texture here; depth is one
            depth: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,

            // multiple textures of the same size can be stored in one texture
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,

            // SAMPLED: tells wgpu that we want to use this texture in shaders;
            // COPY_DST: we want to copy data to this texture
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        // COPY_SRC: copy it to our texture
        let buffer = device
            .create_buffer_mapped(rgba.len(), wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&rgba);

        let mut encoder = device.create_command_encoder(&Default::default());

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                row_pitch: 4 * dimensions.0,
                image_height: dimensions.1,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        let cmd_buffer = encoder.finish();

        // TextureView: offers us a *view* into our texture
        let view = texture.create_default_view();

        // Sampler: controls how the Texture is *sampled*.
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare_function: wgpu::CompareFunction::Always,
        });

        Ok((
            Texture {
                texture,
                view,
                sampler,
            },
            cmd_buffer,
        ))
    }
}
