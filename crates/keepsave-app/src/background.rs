//! The animated background, drawn by a wgpu shader through an egui paint
//! callback before anything else on the frame.

use std::time::Duration;

use eframe::{
  egui::{self, Color32, Pos2},
  egui_wgpu::{self, CallbackResources, CallbackTrait, ScreenDescriptor, wgpu},
};

use crate::theme;

/// The blobs drift slowly, so this frame rate looks smooth and stays cheap.
const FRAME_INTERVAL: Duration = Duration::from_millis(33);
const UNFOCUSED_FRAME_INTERVAL: Duration = Duration::from_millis(100);

/// GPU objects, stored in the renderer's callback resources.
struct Resources {
  pipeline: wgpu::RenderPipeline,
  uniforms: wgpu::Buffer,
  bind_group: wgpu::BindGroup,
  srgb_target: bool,
}

pub struct Background {
  gpu: bool,
  /// The window is transparent over the OS backdrop.
  transparent: bool,
}

impl Background {
  pub fn new(cc: &eframe::CreationContext<'_>, transparent: bool) -> Self {
    let Some(render_state) = cc.wgpu_render_state.as_ref() else {
      return Self { gpu: false, transparent: false };
    };
    let resources = Resources::new(&render_state.device, render_state.target_format);
    render_state.renderer.write().callback_resources.insert(resources);
    Self { gpu: true, transparent }
  }

  pub fn is_transparent(&self) -> bool {
    self.transparent
  }

  /// `animated` off draws a still frame and only repaints on input.
  pub fn paint(&self, ui: &egui::Ui, animated: bool) {
    let rect = ui.ctx().content_rect();
    let painter = ui.painter();
    if !self.transparent {
      painter.rect_filled(rect, 0.0, theme::BASE);
    }
    if self.gpu {
      let time = if animated { ui.input(|input| input.time) as f32 } else { 0.0 };
      painter.add(egui_wgpu::Callback::new_paint_callback(rect, BackgroundCallback { time, transparent: self.transparent }));
    } else {
      paint_fallback(painter, rect);
    }

    let (focused, minimized) = ui.input(|input| (input.viewport().focused.unwrap_or(true), input.viewport().minimized == Some(true)));
    if animated && !minimized {
      ui.ctx().request_repaint_after(if focused { FRAME_INTERVAL } else { UNFOCUSED_FRAME_INTERVAL });
    }
  }
}

/// Static soft glows for when wgpu is not available.
fn paint_fallback(painter: &egui::Painter, rect: egui::Rect) {
  let glows = [
    (0.22, 0.28, Color32::from_rgba_unmultiplied(77, 77, 242, 26)),
    (0.80, 0.22, Color32::from_rgba_unmultiplied(20, 158, 189, 20)),
    (0.68, 0.86, Color32::from_rgba_unmultiplied(158, 56, 219, 22)),
  ];
  let radius = rect.width().max(rect.height()) * 0.12;
  for (x, y, color) in glows {
    let center = Pos2::new(rect.left() + rect.width() * x, rect.top() + rect.height() * y);
    for step in 1..=6 {
      painter.circle_filled(center, radius * step as f32, color);
    }
  }
}

impl Resources {
  fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("keepsave-background"),
      source: wgpu::ShaderSource::Wgsl(include_str!("background.wgsl").into()),
    });
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("keepsave-background"),
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
        count: None,
      }],
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("keepsave-background"),
      bind_group_layouts: &[Some(&bind_group_layout)],
      immediate_size: 0,
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("keepsave-background"),
      layout: Some(&layout),
      vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[] },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        compilation_options: Default::default(),
        targets: &[Some(target_format.into())],
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      multiview_mask: None,
      cache: None,
    });
    let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("keepsave-background"),
      size: UNIFORMS_SIZE,
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("keepsave-background"),
      layout: &bind_group_layout,
      entries: &[wgpu::BindGroupEntry { binding: 0, resource: uniforms.as_entire_binding() }],
    });

    Self { pipeline, uniforms, bind_group, srgb_target: target_format.is_srgb() }
  }
}

/// `vec2 resolution, f32 time, f32 srgb_target, f32 transparent`, padded
/// to 16-byte alignment.
const UNIFORMS_SIZE: u64 = 32;

struct BackgroundCallback {
  time: f32,
  transparent: bool,
}

impl CallbackTrait for BackgroundCallback {
  fn prepare(
    &self,
    _device: &wgpu::Device,
    queue: &wgpu::Queue,
    screen: &ScreenDescriptor,
    _encoder: &mut wgpu::CommandEncoder,
    resources: &mut CallbackResources,
  ) -> Vec<wgpu::CommandBuffer> {
    if let Some(resources) = resources.get::<Resources>() {
      let [width, height] = screen.size_in_pixels;
      let flag = |on: bool| if on { 1.0 } else { 0.0 };
      let values = [width as f32, height as f32, self.time, flag(resources.srgb_target), flag(self.transparent), 0.0, 0.0, 0.0];
      let bytes: Vec<u8> = values.iter().flat_map(|value| value.to_ne_bytes()).collect();
      queue.write_buffer(&resources.uniforms, 0, &bytes);
    }
    Vec::new()
  }

  fn paint(&self, _info: egui::PaintCallbackInfo, render_pass: &mut wgpu::RenderPass<'static>, resources: &CallbackResources) {
    if let Some(resources) = resources.get::<Resources>() {
      render_pass.set_pipeline(&resources.pipeline);
      render_pass.set_bind_group(0, &resources.bind_group, &[]);
      render_pass.draw(0..3, 0..1);
    }
  }
}
