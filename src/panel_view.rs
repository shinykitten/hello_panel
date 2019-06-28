use image::RgbaImage;
use piston_window::{Event,PressEvent,Button};
use crate::panel_controller::PanelCache;

pub struct PanelView {
    pub image_buffer: RgbaImage,
    cache: PanelCache,
    dirty: bool,

    // Which panel the top left screen pixel is in, and its offset within that panel.
    tl_panel: &'static str,
    tl_offset: (u32, u32),
}

// PanelView is responsible for filling an image buffer with pixels corresponding to the currently
// visible background panels.  The core idea is to keep track of the top-left screen pixel,
// relative to which panel it's in, and derive everything else from there.
impl PanelView {
    pub fn new() -> PanelView {
        PanelView {
            image_buffer: RgbaImage::new(crate::PANEL_WIDTH as u32, crate::PANEL_HEIGHT as u32),
            cache: PanelCache::new(crate::PANEL_WIDTH, crate::PANEL_HEIGHT),
            dirty: true,
            tl_panel: "assets/blue-orange-45.png",
            tl_offset: (0, 0),
        }
    }

    // TODO: Move direct consumption of the piston_window Event out to some piston-specific thing.  It's
    // only there right now to test moving around.
    pub fn update(&mut self, e: &Event) -> bool {
        match e.press_args() {
            Some(Button::Keyboard(key)) => println!("key: {:?}", key),
            _ => (),
        }
        if self.dirty {
            self.dirty = false;
            self.fill_img_buffer();
            return true;
        }
        false
    }

    fn fill_img_buffer(&mut self) {
        let rgba_img: RgbaImage = self.cache.open(self.tl_panel).unwrap();
        for (x, y, pixel) in rgba_img.enumerate_pixels() {
            self.image_buffer.put_pixel(x, y, *pixel);
        }
    }
}
