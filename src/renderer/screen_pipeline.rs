use crate::ui::UI;

pub struct ScreenPipelineDescriptor<'a> {
    pub device: &'a wgpu::Device,
    pub format: wgpu::TextureFormat,
}

pub struct ScreenPipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    egui_renderer: egui_wgpu::Renderer,
}

impl ScreenPipeline {
    pub fn new(descriptor: ScreenPipelineDescriptor) -> Self {
        let shader = descriptor.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Screen Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/screen.wgsl").into()),
        });

        let bind_group_layout =
            descriptor.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Screen bind group layout"),
                entries: &[],
            });

        let bind_group = descriptor.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[],
        });

        let pipeline_layout =
            descriptor.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = descriptor.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Screen Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: descriptor.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                ..Default::default()
            },
            multiview: None,
        });


        let egui_renderer = egui_wgpu::Renderer::new(descriptor.device, descriptor.format, None, 1);

        Self {
            pipeline,
            bind_group,
            egui_renderer,
        }
    }

    pub fn pass(&mut self, descriptor: ScreenPassDescriptor) {
        let egui_full_output = descriptor.ui.take_full_output();

        let egui_ctx = descriptor.ui.ctx();

        if let Some(egui_full_output) = &egui_full_output {
            for (id, image_delta) in &egui_full_output.textures_delta.set {
                self.egui_renderer.update_texture(descriptor.device, descriptor.queue, *id, image_delta)
            }
        }

        let egui_data = egui_full_output.map(|egui_full_output| (
            egui_ctx.tessellate(egui_full_output.shapes, egui_full_output.pixels_per_point),
            egui_wgpu::ScreenDescriptor { 
                size_in_pixels: [
                    descriptor.surface_config.width,
                    descriptor.surface_config.height
                ],
                pixels_per_point: egui_full_output.pixels_per_point
            }
        ));

        if let Some((egui_primitives, screen_descriptor)) = &egui_data {
            self.egui_renderer.update_buffers(
                descriptor.device,
                descriptor.queue,
                descriptor.encoder,
                egui_primitives,
                screen_descriptor,
            );
        }

        let mut render_pass = descriptor.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: descriptor.output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..6, 0..1);

        if let Some((egui_primitives, screen_descriptor)) = &egui_data {
            self.egui_renderer.render(&mut render_pass, egui_primitives, screen_descriptor);
        }
    }
}


pub struct ScreenPassDescriptor<'a> {
    pub surface_config: &'a wgpu::SurfaceConfiguration,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub output_view: &'a wgpu::TextureView,
    pub ui: &'a mut UI,
}
