pub struct VideoSink {
	inner: Option<Vec<u32>>
}

impl VideoSink {
	pub fn new() -> VideoSink {
		VideoSink {
			inner: None
		}
	}
	pub fn consume(self) -> Option<Vec<u32>> {
		self.inner
	}
	pub fn append(&mut self, value: Vec<u32>) {
		self.inner = Some(value);
	}
}
