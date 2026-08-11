pub mod Video{
    use v4l::io::traits::CaptureStream;
use v4l::{buffer::Type};
    use v4l::io::mmap::Stream;
    use v4l::prelude::*;

    pub struct VideoCam{
        webcam: Device,
        stream: Stream<'static> // Implicitly assumed since Program will exit before Stream(videocam) becomes Invalid
    }

    impl VideoCam{
        pub fn new(path: Option<&str>) -> Self{
            let mut dev = Device::with_path(path.unwrap_or("/dev/video0")).expect(&format!("Unable to open device file: {}",path.unwrap_or("/dev/video0")));
            let k = MmapStream::with_buffers(&mut dev, Type::VideoCapture,4).expect("Unable to crerate buffer stream");
            Self { webcam:dev,stream:k}
        }

        pub fn next(&mut self) -> (&[u8],&v4l::buffer::Metadata){
            self.stream.next().unwrap()
        }
    }
   


pub fn x(){
use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;

let mut dev = Device::new(0).expect("Failed to open device");

let mut stream =
    MmapStream::with_buffers(&mut dev, Type::VideoCapture, 4).expect("Failed to create buffer stream");

loop {
    let (buf, meta) = stream.next().unwrap();
    println!(
        "Buffer size: {}, seq: {}, timestamp: {}",
       buf.len(),
       meta.sequence,
       meta.timestamp
   );
}

}

}