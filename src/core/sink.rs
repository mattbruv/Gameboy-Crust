pub struct VideoSink {
	inner: Option<Vec<u8>>
}

impl VideoSink {
	pub fn new() -> VideoSink {
		VideoSink {
			inner: None
		}
	}
	pub fn consume(self) -> Option<Vec<u8>> {
		self.inner
	}
	pub fn append(&mut self, value: Vec<u8>) {
		self.inner = Some(value);
	}
}