pub fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    color_format: wgpu::TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    topology: wgpu::PrimitiveTopology,
    shader: wgpu::ShaderModuleDescriptor,
) -> wgpu::RenderPipeline {
    let label = shader.label;
    let shader = device.create_shader_module(shader);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vertex_layouts,
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState {
                    alpha: wgpu::BlendComponent::REPLACE,
                    color: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),

        primitive: wgpu::PrimitiveState {
            topology,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RESTERIZATION
            conservative: false,
        },
        depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
            format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
