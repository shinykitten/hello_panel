use image::RgbaImage;

pub struct PanelCache {
    panel_width: usize,  // Needed when creating new panels; assigned at new, never updated.
    panel_height: usize,  // Needed when creating new panels; assigned at new, never updated.
    next_stamp: u32,  // An incrementing counter, used to know when this panel was last touched.
}

// PanelCache speaks in terms of filenames.  It handles the job of caching panels so you're not
// reading from disk every time you load a panel.  It evicts the least recently used panels.
impl PanelCache {
    pub fn new(panel_width: usize, panel_height: usize) -> PanelCache {
        PanelCache {
            panel_width: panel_width,
            panel_height: panel_height,
            next_stamp: 0,
        }
    }

    pub fn open(&mut self, open_path: &str) -> Result<RgbaImage, image::ImageError> {
        match image::open(open_path) {
            Err(e) => Err(e),
            Ok(img) => Ok(img.to_rgba()),
        }
    }
}

pub enum PanelTransition {
    ShareScreen, SnapToScreenEdge,
}

pub struct PanelGraphEdge {
    from: String,  // Asset path name.
    to: String,
    transition: PanelTransition,
}

pub struct PanelGraph {

}
