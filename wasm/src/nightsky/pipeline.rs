use std::{ffi::c_void, ptr::NonNull};

use raw_window_handle::{WebCanvasWindowHandle, WebDisplayHandle};
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use wgpu::{util::DeviceExt, SurfaceTargetUnsafe};

use super::star::Star;

const SAMPLE_COUNT: u32 = 8;

/// Create a new wgpu Instance
/// # Info
/// The instance is the entry point to the wgpu API.
/// It is used to create adapters, surfaces, and other resources.
pub fn create_instance() -> wgpu::Instance {
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::GL,
        ..Default::default()
    })
}

/// Create a new wgpu Surface
/// # Info
/// The surface is used to present rendered frames to a window.
/// It is what the renderer will draw to.
pub fn create_surface(
    instance: &wgpu::Instance,
    canvas: &HtmlCanvasElement,
) -> wgpu::Surface<'static> {
    let value: &JsValue = &canvas;
    let obj: NonNull<c_void> = NonNull::from(value).cast();
    let handle = WebCanvasWindowHandle::new(obj);
    let display = WebDisplayHandle::new();

    let target = SurfaceTargetUnsafe::RawHandle {
        raw_display_handle: raw_window_handle::RawDisplayHandle::Web(display),
        raw_window_handle: raw_window_handle::RawWindowHandle::WebCanvas(handle),
    };

    unsafe { instance.create_surface_unsafe(target) }.unwrap()
}

/// Request an adapter from the instance to fit the surface
/// # Info
/// An adapter is the actual graphics processor (GPU) that you are interacting with.
pub async fn request_adapter(
    instance: &wgpu::Instance,
    surface: &wgpu::Surface<'_>,
) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap()
}

pub async fn request_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web, we'll have to disable some.
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: None,
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .unwrap()
}

pub fn create_multisampled_frame(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Multisampled Framebuffer"),
        size: wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1, // 2D texture, no array layers
        },
        mip_level_count: 1,         // No mipmaps needed
        sample_count: SAMPLE_COUNT, // Use the desired sample count (e.g., 4 for 4x MSAA)
        dimension: wgpu::TextureDimension::D2,
        format: config.format, // Match the surface format
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // Must be used as a render attachment
        view_formats: &[],     // No additional view formats
    })
}

pub fn configure_surface(
    adapter: &wgpu::Adapter,
    surface: &wgpu::Surface,
    height: u32,
    width: u32,
) -> wgpu::SurfaceConfiguration {
    let surface_caps = surface.get_capabilities(&adapter);
    // Shader code in this tutorial assumes an sRGB surface texture. Using a different
    // one will result in all the colors coming out darker. If you want to support non
    // sRGB surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);
    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width,
        height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    }
}

pub fn begin_render_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    view: &'a wgpu::TextureView,
    multisampled_view: &'a wgpu::TextureView,
    clear_color: wgpu::Color,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: multisampled_view,
            resolve_target: Some(view),
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear_color),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    })
}

pub fn create_star_buffer(device: &wgpu::Device, stars: &[Star]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Star Buffer"),
        contents: bytemuck::cast_slice(stars),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    })
}

pub fn vertex_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Vertex Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("vertex.wgsl").into()),
    })
}

pub fn fragment_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Fragment Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("fragment.wgsl").into()),
    })
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
) -> wgpu::RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let vertex_shader = vertex_shader(device);
    let fragment_shader = fragment_shader(device);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: Some("main"),
            buffers: &[Star::desc()],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            // 3.
            module: &fragment_shader,
            entry_point: Some("main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleStrip,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
