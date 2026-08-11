pub mod Video{
    use v4l::buffer::Type;
    use v4l::io::traits::CaptureStream;
    use v4l::prelude::*;
    use std::sync::{LazyLock, RwLock};

    pub struct VideoCam{
        webcam: Device,
        

    }

    impl VideoCam{
        pub fn new(path: Option<&str>) -> Self{
            Self { webcam: Device::with_path(path.unwrap_or("/dev/video0")).expect(&format!("Unable to open device file: {}",path.unwrap_or("/dev/video0"))) }
        }




    }


    pub static webcam: LazyLock<RwLock<Device>> = LazyLock::new(|| {RwLock::new( Device::with_path("/dev/video0").expect("Unable to open /dev/video0"))});
    let mut stream = MmapStream::with_buffers(&mut webcam, Type::VideoCapture,4);
   






}