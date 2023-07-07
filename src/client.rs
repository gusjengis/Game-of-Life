use crate::wgpu_prog::WGPUComputeProg;
use crate::windowInit;
use crate::wgpu_config::*;
use crate::wgpu_prog;

use crate::wgpu_prog::WGPUProg;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::WindowBuilder, dpi::PhysicalSize,
};

use winit_fullscreen;
use winit_fullscreen::WindowFullScreen;

use chrono::prelude::*;

pub struct Client {
    pub canvas: windowInit::Canvas,
    wgpu_config: WGPUConfig,
    wgpu_prog: WGPUProg,
    last_draw: chrono::DateTime<Local>,
    log_framerate: bool,
    start_time: DateTime<Local>,
    bench_start_time: DateTime<Local>,
    generations: f32,
    temp: f32,
    toggle: bool,
    prev_gen_time: DateTime<Local>,
    cursor_pos: (i32, i32),
    HL: bool,
    genPerFrame: i32,
    prevGen: i32,
    generation: i32,
    xOff: i32,
    yOff: i32,
    scale: f32,
    middle: bool,
    dark: f32
}

impl Client {
    pub async fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        // window.set_cursor_visible(false);
        window.set_title("Game of Life");

        let canvas = windowInit::Canvas::new(window);
        let wgpu_config = WGPUConfig::new(&canvas).await;//pollster::block_on(
        let last_draw = Local::now();
        let log_framerate = false;
        let wgpu_prog = WGPUProg::new(&wgpu_config);
        let start_time = Local::now();
        let bench_start_time = Local::now();
        let generations = 100.0;//256.1;
        let temp = 1.0;//256.1;
        let toggle = false;
        let prev_gen_time = Local::now();
        let cursor_pos = (0, 0);
        let HL = false;
        let genPerFrame = 1; 
        let prevGen = 0;
        let generation = 0;
        let xOff = 0;
        let yOff = 0;
        let scale = 8.0;
        let middle = false;
        let dark = 0.0;
        let mut client = Client {
            canvas,
            wgpu_config,
            last_draw,
            log_framerate,
            wgpu_prog,
            start_time,
            bench_start_time,
            temp,
            prevGen,
            generations,
            toggle,
            prev_gen_time,
            cursor_pos,
            HL,
            genPerFrame,
            generation,
            xOff,
            yOff,
            scale,
            middle,
            dark
        };

        //Start Event Loop
        // client.randomize();
        

        event_loop.run(move |event, _, control_flow| match event { 
            Event::WindowEvent {
                ref event,
                window_id,
            } 
            if window_id == client.canvas.window.id() => {
                if !client.input(event) {
                    match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        client.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
    
                        client.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }}
            Event::RedrawRequested(window_id) if window_id == client.canvas.window.id() => {
                // client.update();
                match client.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => client.resize(client.canvas.size.clone()),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                client.canvas.window.request_redraw();
            }
            _ => {}
        });

        return client;
    }

    // pub fn window(&self) -> &Window {
    //     &self.window
    // }

    // fn update(&mut self) {
    //     // remove todo!()
    // }

    fn cursorToPixel(&self) -> Option<(u32, u32)> {
        let cursor = (self.cursor_pos.0 - self.xOff, self.cursor_pos.1 - self.yOff);
        let dim = &self.wgpu_prog.shader_prog.tex2.dimensions;//(11584, 11584);
        let windowDim = self.wgpu_config.size;
        let int_scale = self.scale as f32;//windowDim.height/dim.1;
        let xOff = ((windowDim.width as f32) - (dim.0 as f32)*(int_scale))/2.0; 
        let yOff = ((windowDim.height as f32) - (dim.1 as f32)*(int_scale))/2.0;
        let coords = (((dim.0 as f32)*(cursor.0 as f32-xOff )/(dim.0 * int_scale as u32) as f32) as u32, ((dim.1 as f32)*(cursor.1 as f32-yOff )/(dim.1 * int_scale as u32 ) as f32) as u32);
        if(coords.0 < 0 || coords.0 > dim.0 - 1 || coords.1 < 0 || coords.1 > dim.1 - 1){
            return None;
        }
        return Some(coords);
    }

    fn writeToTex(&self, pixel: (u32, u32), color: &[u8]) {
        let mut texture = &self.wgpu_prog.shader_prog.tex2.texture;
        let mut dimensions = &self.wgpu_prog.shader_prog.tex2.dimensions;
        if(self.wgpu_prog.shader_prog.use1){
            texture = &self.wgpu_prog.shader_prog.tex1.texture;
            dimensions = &self.wgpu_prog.shader_prog.tex1.dimensions;
        }
        self.wgpu_config.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: pixel.0,
                    y: pixel.1,
                    z: 0
                },
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            color,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some((4 * dimensions.0) as u32),
                rows_per_image: Some((dimensions.1) as u32),
            },
            wgpu::Extent3d {
                width: 1, 
                height: 1,
                depth_or_array_layers: 1,
            }
        );
    }
    fn randomize(&mut self){
        for x in 0..self.wgpu_prog.shader_prog.tex1.dimensions.0-1 {
            for y in 0..self.wgpu_prog.shader_prog.tex1.dimensions.1-1 {
                let BorW = (rand::random::<f32>()).round();
                if(!self.toggle){
                    self.writeToTex((x, y), &[(255.0*BorW) as u8, (255.0*BorW) as u8, (255.0*BorW) as u8, (255.0*BorW) as u8]);
                }
            }
        }
    }

    pub fn computeGOLGen(&mut self){
        // while(true){
            // println! ("test");
            // println! ("{}", self.toggle);

            if(self.toggle && Local::now().timestamp_millis() - self.prev_gen_time.timestamp_millis() >= self.generations as i64){
                for i in 0..self.genPerFrame {
                    // let start = Local::now();

                    self.wgpu_prog.shader_prog.compute(&self.wgpu_config);
                    self.wgpu_prog.swap(&self.wgpu_config);
                    self.prev_gen_time = Local::now();
                    self.generation += 1;
                    // println! ("{}", ((Local::now().timestamp_micros() - start.timestamp_micros()) as f32/1000.0));

                    // println! ("{}", 1000.0/((Local::now().timestamp_micros() - start.timestamp_micros()) as f32/1000.0));

                }
            }
        // }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            // let size = self.canvas.window.
            self.canvas.updateSize(new_size);
            self.wgpu_config.config.width = new_size.width;
            self.wgpu_config.config.height = new_size.height;
            self.wgpu_config.size = new_size;
            // #[cfg(target_arch = "wasm32")]{
            //     self.canvas.window.set_inner_size(new_size);
            // }
            // log::warn!("{}", new_size.width);
            // self.wgpu_prog.dim_uniform.updateUniform(&self.wgpu_config.device, bytemuck::cast_slice(&[new_size.width as f32, new_size.height as f32, new_size.width as f32, new_size.height as f32]));
            self.wgpu_config.surface.configure(&self.wgpu_config.device, &self.wgpu_config.config);

            let windowDim = self.wgpu_config.size;
            let dim = &self.wgpu_prog.shader_prog.tex2.dimensions;
            let int_scale = self.scale as f32;//(windowDim.height/dim.1) as f32;
            
            if(self.temp > int_scale - 1.0){
                self.temp = int_scale - 1.0;
            }
            self.wgpu_prog.dim_uniform.updateUniform(&self.wgpu_config.device, bytemuck::cast_slice(
                &[self.wgpu_config.size.width as f32,
                  self.wgpu_config.size.width as f32, 
                  self.wgpu_config.size.height as f32,
                  self.wgpu_config.size.height as f32,
                  self.xOff as f32,
                  self.yOff as f32,
                  self.scale as f32,
                  self.dark as f32]
            ));
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            // WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Right, .. } => {
            //     let mut coords = self.cursorToPixel();
            //     match coords {
            //         Some(value) => {
            //             if(!self.toggle){
            //                 let pixel = coords.unwrap();
            //                 self.writeToTex(pixel, &[0 as u8, 0 as u8, 0 as u8, 0 as u8]);
            //             }
            //             return true;
            //         }
            //         None => {return true;}
            //     }
            // },h 
            // WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
            //     let mut coords = self.cursorToPixel();
            //     match coords {
            //         Some(value) => {
            //             if(!self.toggle){
            //                 let pixel = coords.unwrap();
            //                 self.writeToTex(pixel, &[255 as u8, 255 as u8, 255 as u8, 255 as u8]);
            //             }
            //             return true;
            //         }
            //         None => {return true;}
            //     }
            // },
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Middle, .. } => {
                self.middle = true;
                return true;
            },
            WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Middle, .. } => {
                self.middle = false;
                return true;
            }
            WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.scale = (self.scale as f32*((2 as f32).powf(*y)));
                    }
                    _ => {}
                }
                if(self.scale < 0.125){ self.scale = 0.125; }
                if(self.temp > self.scale as f32 - 1.0){
                    self.temp = self.scale as f32 - 1.0;
                }
                if(self.temp < 0.0){
                    self.temp = 0.0;
                }
                return true;

            },
            WindowEvent::CursorMoved { position, .. } => {
                let delta = (position.x as i32 - self.cursor_pos.0, position.y as i32 - self.cursor_pos.1);
                self.cursor_pos = (position.x as i32, position.y as i32);
                if(self.middle){
                    // let aspect = self.wgpu_config.size.width as f32/self.wgpu_config.size.height as f32;//println!("{}", self.wgpu_config.size.width as f32/self.wgpu_config.size.height as f32);
                    self.xOff += (delta.0 as f32) as i32;
                    self.yOff += (delta.1 as f32) as i32;
                }
                // self.clear_color = wgpu::Color {
                //     r: position.x as f64 / self.size.width as f64,
                //     g: position.y as f64 / self.size.height as f64,
                //     b: (position.x + position.y)as f64 / 2.0* self.size.width as f64,
                //     a: 1.0,
                // };
                // self.cursor_uniform.updateUniform(&self.device, bytemuck::cast_slice(&[position.x as f32, position.y as f32, position.x as f32, position.y as f32]));
                return true;
            },
            WindowEvent::KeyboardInput { input, .. } => {
                match input {
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::F11),
                        state: ElementState::Pressed,
                        ..
                    } => {
                            self.canvas.window.toggle_fullscreen();
                            return true;
                        },
                    KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Space),
                            state: ElementState::Pressed,
                            ..
                        } => {
                                // self.temp = 1.0;
                                self.start_time = Local::now();
                                self.toggle = !self.toggle;
                                return true;
                            },
                    KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::R),
                            state: ElementState::Pressed,
                            ..
                        } => {
                                // self.temp = 1.0;
                                self.start_time = Local::now();
                                self.wgpu_prog.shader_prog = WGPUComputeProg::new(&self.wgpu_config);
                                self.toggle = false;
                                self.generation = 0;
                                return true;
                            },
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::H),
                        state: ElementState::Pressed,
                        ..
                    } => {
                            // self.temp = 1.0;
                            self.xOff = 0;
                            self.yOff = 0;
                            return true;
                        },
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::C),
                        state: ElementState::Pressed,
                        ..
                    } => {
                            // self.temp = 1.0;
                            self.start_time = Local::now();
                            self.wgpu_prog.shader_prog = WGPUComputeProg::new(&self.wgpu_config);
                            self.toggle = false;
                            self.generation = 0;
                            self.wgpu_prog.shader_prog.clearTextures(&self.wgpu_config);
                            return true;
                        },
                    
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::L),
                        state: ElementState::Pressed,
                        ..
                    } => {
                            self.HL = !self.HL;
                            return true;
                        },
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::D),
                        state: ElementState::Pressed,
                        ..
                    } => {
                            if(self.dark == 0.0){
                                self.dark = 1.0;
                            } else {
                                self.dark = 0.0
                            }
                            return true;
                        },
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        state: ElementState::Pressed,
                        ..
                    } => {
                            let windowDim = self.wgpu_config.size;
                            let dim = &self.wgpu_prog.shader_prog.tex2.dimensions;
                            // let int_scale = self.scale as f32;//(windowDim.height/dim.1) as f32;
                            self.temp += 1.0;
                            
                            if(self.temp > self.scale as f32 - 1.0){
                                self.temp = self.scale as f32 - 1.0;
                            }
                            return true;
                        },
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        state: ElementState::Pressed,
                        ..
                    } => {
                            self.temp -= 1.0;
                            if(self.temp < 0.0){
                                self.temp = 0.0;
                            }
                            return true;
                        },
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        state: ElementState::Pressed,
                        ..
                    } => {
                            if(self.genPerFrame > 1){
                                self.genPerFrame -= 1;
                            } else {
                                self.generations += 10.0;
                            }
                            return true;
                        },
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        state: ElementState::Pressed,
                        ..
                    } => {
                            self.generations -= 10.0;
                            if(self.generations < 0.0){
                                self.generations = 0.0;
                                self.genPerFrame += 1;
                            }
                            return true;
                        },
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::F3),
                        state: ElementState::Pressed,
                        ..
                    } => {
                            self.log_framerate = !self.log_framerate;
                            Client::clearConsole();
                            return true;
                        },
                    _ => false
                }
            },
            _ => false,
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // log::debug!("render");

        //RANDOM CLEAR COLOR EACH FRAME
        // let mut rng = rand::thread_rng();
        // self.clear_color = wgpu::Color {
        //     r: rng.gen::<f64>()/1.0,
        //     g: rng.gen::<f64>()/1.0,
        //     b: rng.gen::<f64>()/1.0,
        //     a: 1.0,
        // };
        
        
        
        let mut time = 1;//Local::now().timestamp_millis() - self.start_time.timestamp_millis();
        if(!self.HL){
            time = 0;
        }
        
        self.wgpu_prog.dim_uniform.updateUniform(&self.wgpu_config.device, bytemuck::cast_slice(
            &[self.wgpu_config.size.width as f32,
              time as f32, 
              self.wgpu_config.size.height as f32,
              self.temp,
              self.xOff as f32,
              self.yOff as f32,
              self.scale as f32,
              self.dark as f32]
        ));   

        // if(self.temp < 256.5){
        //     self.temp += 0.2;
        // } 
        // self.wgpu_prog.time_uniform.updateUniform(&self.wgpu_config.device, bytemuck::cast_slice(
        //     &[time as f32,
        //       time as f32, 
        //       time as f32,
        //       time as f32]
        // ));

        #[cfg(target_arch = "wasm32")] {
            let w = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
            let h = web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;
            if(!(self.canvas.size.width == w && self.canvas.size.height == h)){
                self.resize(winit::dpi::PhysicalSize::new(w,h));
            }
        }

        let output = self.wgpu_config.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        
            self.computeGOLGen();
            
        

        let mut encoder = self
            .wgpu_config.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.wgpu_prog.clear_color),
                            store: true,
                        }
                    })
                ],
                depth_stencil_attachment: None,
            });

            self.wgpu_prog.shader_prog.tex1.setBinding(&self.wgpu_config, 5, false);
        
            // NEW!
            render_pass.set_pipeline(&self.wgpu_prog.render_pipeline); // 2.
            render_pass.set_bind_group(0, &self.wgpu_prog.tex1.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.wgpu_prog.dim_uniform.bind_group, &[]);
            // render_pass.set_bind_group(2, &self.wgpu_prog.time_uniform.bind_group, &[]);
            render_pass.set_bind_group(2, &self.wgpu_prog.tex2.diffuse_bind_group, &[]);

            let texSelector = self.wgpu_prog.shader_prog.use1;
            if(texSelector){
                render_pass.set_bind_group(3, &self.wgpu_prog.shader_prog.tex1.diffuse_bind_group, &[]);

            } else {
                render_pass.set_bind_group(3, &self.wgpu_prog.shader_prog.tex2.diffuse_bind_group, &[]);

            }

            // render_pass.set_bind_group(1, &self.cursor_uniform.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.wgpu_prog.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.wgpu_prog.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..6 as u32, 0, 0..1); // 3.

            
        }
    
        // submit will accept anything that implements IntoIter
        self.wgpu_config.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        let now = Local::now();
        if(self.log_framerate){
            
            let time_since = (now.timestamp_millis() - self.bench_start_time.timestamp_millis()) as f32/1000.0;
            if(time_since >= 0.25){
                Client::clearConsole();
                #[cfg(not(target_arch = "wasm32"))] {
                    println!("FPS: {}", 1000000.0/(now.timestamp_micros() - self.last_draw.timestamp_micros()) as f32);
                }
                #[cfg(target_arch = "wasm32")] {
                    log::warn!("FPS: {}", 1000000.0/(now.timestamp_micros() - self.last_draw.timestamp_micros()) as f32);
                }
                println!("Generations/s: {}, Total Generations: {}", (self.generation - self.prevGen) as f32/time_since, self.generation);
                println!("Generations/Update: {}, Time Between Updates(ms): {}", self.genPerFrame as f32, self.generations);
                println!("Scale: {}, (xOff, yOff): ({}, {})", self.scale as f32, self.xOff, self.yOff);
                self.prevGen = self.generation;
                self.bench_start_time = Local::now();
                
            }
            
        }
        
        self.last_draw = now;

        Ok(())
    }

    fn clearConsole(){
        #[cfg(not(target_arch = "wasm32"))] {
            print!("\x1B[2J\x1B[1;1H");
        }
        #[cfg(target_arch = "wasm32")] {
            web_sys::console::clear();
        }
    }
}