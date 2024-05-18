pub mod screen_pipeline;

use std::sync::Arc;

use winit::window::Window;

use crate::ui::UI;

use self::screen_pipeline::{ScreenPassDescriptor, ScreenPipeline, ScreenPipelineDescriptor};

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    screen_pipeline: ScreenPipeline,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window)
            .expect("Failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::BGRA8UNORM_STORAGE,
                required_limits: wgpu::Limits {
                    ..wgpu::Limits::downlevel_defaults()
                        .using_resolution(adapter.limits())
                }
            },
            None,
        )
        .await
        .expect("Failed to create device");

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let screen_pipeline = ScreenPipeline::new(
            ScreenPipelineDescriptor {
                device: &device,
                format: surface_format,
            }
        );

        Renderer {
            surface,
            device,
            queue,
            surface_config,
            screen_pipeline
        }
    }

    pub fn render(&mut self, ui: &mut UI) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let output_view = output.texture.create_view(&wgpu::TextureViewDescriptor { 
            ..wgpu::TextureViewDescriptor::default()
        });

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.screen_pipeline.pass(ScreenPassDescriptor {
            surface_config: &self.surface_config,
            encoder: &mut encoder,
            device: &self.device,
            queue: &self.queue,
            output_view: &output_view,
            ui,
        });

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }
}
