pub struct FontMetrics {
    pub size: u32,
    pub internal_leading: u32,
    pub ascent: u32,
    pub descent: u32,
    pub external_leading: u32,
}

impl FontMetrics {
    pub fn width(&self) -> u32 {
        self.size / 2
    }
    pub fn height(&self) -> u32 {
        self.internal_leading + self.ascent + self.descent
    }
    pub fn row_height(&self) -> u32 {
        self.height() + self.external_leading
    }
}
