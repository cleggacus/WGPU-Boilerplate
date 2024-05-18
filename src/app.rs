use std::sync::Arc;

use winit::{dpi::PhysicalSize, event::{Event, WindowEvent}, event_loop::{EventLoop, EventLoopWindowTarget}, window::{Window, WindowBuilder}};

use crate::{renderer::Renderer, ui::UI};

pub struct App {
    window: Arc<Window>,
    renderer: Renderer,
    ui: UI,
    event_loop: Option<EventLoop<()>>,
}

impl App {
    pub async fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();

        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Rays do be going brrrrr")
                .build(&event_loop).unwrap()
        );

        let renderer = Renderer::new(window.clone()).await;
        let ui = UI::new(window.clone());

        Self {
            window: window.clone(),
            renderer,
            ui,
            event_loop: Some(event_loop),
        }
    }

    pub fn run(&mut self) {
        let event_loop = self.event_loop.take().unwrap();

        event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    self.handle_window_event(event, elwt);
                },
                _ => {}
            }
        }).unwrap();
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(&mut self.ui)
    }

    fn update(&mut self) {
        self.ui.update();
    }

    fn handle_redraw_requested(&mut self, elwt: &EventLoopWindowTarget<()>) {
        self.update();

        match self.render() {
            Ok(_) => {},
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) =>
                self.resize(self.window.inner_size()),
            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
            Err(wgpu::SurfaceError::Timeout) => {},
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.renderer.resize(new_size.width, new_size.height);
    }

    fn handle_window_event(&mut self, event: &WindowEvent, elwt: &EventLoopWindowTarget<()>) {
        let event_response = self.ui.egui_state_mut()
            .on_window_event(&self.window, event);

        if event_response.repaint {
            self.window.request_redraw();
        }

        match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::Resized(physical_size) => self.resize(*physical_size),
            WindowEvent::RedrawRequested => self.handle_redraw_requested(elwt),
            WindowEvent::KeyboardInput { .. } |
            WindowEvent::MouseWheel { .. } |
            WindowEvent::MouseInput { .. } |
            WindowEvent::CursorMoved { .. } => 
                {}
                // self.input_manager.window_update(event, event_response.consumed),
            _ => {}
        }
    }
}

