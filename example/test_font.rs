use font_kit::source::SystemSource;
use font_kit::canvas::{Canvas, RasterizationOptions, Format};
use font_kit::hinting::HintingOptions;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::Vector2I;

fn main(){
    println!("---test font---");
    font_kit_text();
}

pub fn font_kit_text(){
    let font = SystemSource::new()
        .select_by_postscript_name("ArialMT")
        .unwrap()
        .load()
        .unwrap();

    let glyph_id = font.glyph_for_char('L').unwrap();

    let size =  Vector2I::new(32, 32);
    let transform = Transform2F::default();

    // let raster_rect = font
    //     .raster_bounds(
    //         glyph_id,
    //         32.0,
    //         transform,
    //         HintingOptions::Vertical(32.0),
    //         RasterizationOptions::SubpixelAa,
    //     )
    //     .unwrap();

    let mut canvas = Canvas::new(size, Format::Rgb24);
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        32.0,
        Transform2F::from_translation(size.to_f32()) *transform,
        HintingOptions::Vertical(32.0),
        RasterizationOptions::SubpixelAa,
    )
        .unwrap();
    println!("{:#?}",canvas.pixels);
}