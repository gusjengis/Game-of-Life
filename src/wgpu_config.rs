use crate::windowInit;
use crate::wgpu_structs::*;
use wgpu::util::DeviceExt;

pub struct WGPUConfig {
    #[allow(dead_code)]
    pub instance: wgpu::Instance,
    #[allow(dead_code)]
    pub adapter: wgpu::Adapter,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>

    // dim_uniform: Uniform,
    // cursor_uniform: Uniform,


}

impl WGPUConfig {
    // Creating some of the wgpu types requires async code
    
    pub async fn new(canvas: &windowInit::Canvas) -> Self {
        
        let size = canvas.size;

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        // # Safety
        //
        // The surface needs to live as long as the canvas that created it.
        // State owns the canvas so this should be safe.
        let surface = unsafe { instance.create_surface(&canvas.window) }.unwrap();

        

        #[cfg(not(target_arch="wasm32"))] 
        let adapter = instance
        .enumerate_adapters(wgpu::Backends::all())
        .filter(|adapter| {
            // Check if this adapter supports our surface
            adapter.is_surface_supported(&surface)
        })
        .next()
        .unwrap();

        #[cfg(target_arch="wasm32")] 
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        // let descriptor = wgpu::DeviceDescriptor {
        //     features: wgpu::Features::empty(),
        //     limits: wgpu::Limits {
        //         max_compute_workgroups_per_dimension: 65535,
        //         ..Default::default()
        //     },
        //     label: None,
        // };

        let (device, queue) = adapter.request_device( //&descriptor,
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_defaults()
                } else {
                    wgpu::Limits::downlevel_defaults()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())// this line is sus, changed f.describe().srgb to f.is_srgb(), describe was not a thing
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
    

 

        
         
        Self {
            instance,
            adapter,
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    

    
}



