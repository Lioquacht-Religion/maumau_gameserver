
/*use beryllium::*;
use ogl33::*;


pub fn main(){
    let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");

   sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
   sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
   sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
   #[cfg(target_os = "macos")]
   {
       sdl
           .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::Forward::ForwardCompatible)
           .unwrap();
    }

   let win = sdl
       .create_gl_window(
           "Hello Window",
           WindowPosition::Centered,
           800,
           600,
           WindowFlags::Shown,
        )
        .expect("couldn't make a window and context");

   'main_loop: loop {
       while let Some(event) = sdl.poll_events().and_then(Result::ok) {
           match event {
               Event::Quit(_) => break 'main_loop,
               _ => (),
           }
       }
   }


    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name));
        glClearColor(0.2, 0.3, 0.3, 1.0);
    }

    unsafe {
        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
    }
    unsafe{
        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
    }

    type Vertex = [f32; 3];
    const VERTICES: [Vertex; 3] =
        [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

    unsafe {
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            GL_STATIC_DRAW,
            );
    }

}

fn example_sdl2(){
    let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");

   sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
   sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
   sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
   #[cfg(target_os = "macos")]
   {
       sdl
           .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::Forward::ForwardCompatible)
           .unwrap();
    }

   let _win = sdl
       .create_gl_window(
           "Hello Window",
           WindowPosition::Centered,
           800,
           600,
           WindowFlags::Shown,
        )
        .expect("couldn't make a window and context");

   'main_loop: loop {
       while let Some(event) = sdl.poll_events().and_then(Result::ok) {
           match event {
               Event::Quit(_) => break 'main_loop,
               _ => (),
           }
       }
   }


}*/
