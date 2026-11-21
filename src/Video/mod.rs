use v4l::io::traits::CaptureStream;
use v4l::{buffer::Type};
use v4l::io::mmap::Stream;
use v4l::prelude::*;

pub struct VideoCam{
    webcam: Device,
    stream: Stream<'static>, // Implicitly assumed since Program will exit before Stream(videocam) becomes Invalid
}

impl VideoCam{
    pub fn new(path: Option<&str>) -> Self{
        let mut dev = Device::with_path(path.unwrap_or("/dev/video0")).expect(&format!("Unable to open device file: {}",path.unwrap_or("/dev/video0")));
        let k = MmapStream::with_buffers(&mut dev, Type::VideoCapture,4).expect("Unable to create buffer stream");
        Self { webcam:dev,stream:k}
    }

    pub fn next(&mut self) -> (&[u8],&v4l::buffer::Metadata){ self.stream.next().unwrap() }
}