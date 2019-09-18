use piston_window::{EventLoop, PistonWindow, WindowSettings};
use piston_window::{Glyphs, Image, Text, Texture, TextureSettings};
use graphics::Transformed;
use fps_counter::FPSCounter;

mod panel_view;
mod panel_controller;

const PANEL_WIDTH: usize = 1280;
const PANEL_HEIGHT: usize = 720;

fn main() {
    // Window contains a factory, which is used for creating (and owning) all kinds of GPU
    // resources, like shaders and (most importantly for us) textures.
    let mut window: PistonWindow = WindowSettings::new("hello panel", [PANEL_WIDTH as u32, PANEL_HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .expect("unable to create window");
    window.set_max_fps(120);

    // A Texture object is just a handle to a resource in the graphics backend, so to do anything
    // with it, you have to go through its methods, which all take a window.encoder (because since
    // window has the resources, only it can manipulate them.)  Texture only has one interesting
    // method for us, which is Update.
    let mut background_tex = Texture::from_memory_alpha(
        &mut window.factory,  // Mutable because window creates and stores resources on itself.
        &[0; PANEL_WIDTH * PANEL_HEIGHT],
        PANEL_WIDTH as u32,
        PANEL_HEIGHT as u32,
        &TextureSettings::new()).unwrap();

    // A Texture represents the GPU resource, not how it gets displayed on screen.  For that, we
    // need an Image.  Idea: create a Texture larger than the screen resolution, so it wouldn't
    // need to get updated as frequently, and use an Image to show different parts.
    let image = Image::new().src_rect([0.0, 0.0, PANEL_WIDTH as f64, PANEL_HEIGHT as f64]);

    // Glyphs::new returns a GlyphCache.  GlyphCache replicates the factory pattern we see on
    // window (it creates Textures for each glyph, manages them internally.)  The downside to this
    // is that it needs access to a window.factory because only window.factory can create Texture
    // resources. But we can't just give the GlyphCache a reference to window.factory because we'll
    // need window later for other things, so we clone it.  I wonder if that's what they intended.
    let mut glyphs = Glyphs::new(
        "assets/FiraSans-Regular.ttf", window.factory.clone(), TextureSettings::new()).unwrap();
    let mut fps = FPSCounter::new();

    let mut pv = panel_view::PanelView::new();
    while let Some(e) = window.next() {
        if pv.update(&e) {
            background_tex.update(&mut window.encoder, &pv.image_buffer).unwrap();
        }

        window.draw_2d(&e, |c, g| {
            graphics::clear([1.0, 0.0, 0.5, 1.0], g);
            image.draw(&background_tex, &c.draw_state, c.transform, g);

            // FPS.
            Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                &format!("{:}", fps.tick()).to_string(),
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(10.0, PANEL_HEIGHT as f64 - 10.0), g
            ).unwrap();
        });
    }
}
