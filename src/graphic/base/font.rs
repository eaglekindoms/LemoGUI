use ab_glyph::*;

use crate::graphic::base::color::RGBA;

#[deprecated]
pub fn draw_text(f_scale: f32, font_color: RGBA, text: &str) -> (u32, u32, Vec<u8>) {
    let font = FontRef::try_from_slice(include_bytes!("../../../shader_c/SourceHanSansCN-Regular.otf")).unwrap();
    default_draw_text(font, f_scale, font_color, text)
}


pub fn default_draw_text(font: FontRef, font_scale: f32, font_color: RGBA, text: &str) -> (u32, u32, Vec<u8>) {

    // The font size to use
    let scale = PxScale::from(font_scale);
    let scaled_font = font.as_scaled(scale);
    let mut glyphs = Vec::new();
    layout_paragraph(scaled_font, point(20.0, 20.0), 9999.0, text, &mut glyphs);

    // Use a dark red colour
    let colour = font_color.to_u8();
    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };

    // Create a new rgba image with some padding
    // let mut image = DynamicImage::new_rgba8(glyphs_width + 20, glyphs_height-15).to_rgba8();
    let size = (glyphs_width + 20) * (glyphs_height);
    let mut bufs1 = vec![0; (size * 4) as usize];

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                // println!("x: {} y: {}", x, y);
                let index = x + bounds.min.x as u32 - 20 + (glyphs_width + 20) * (y + bounds.min.y as u32 - 29);
                bufs1[(index * 4) as usize] = colour.0;
                bufs1[(index * 4) as usize + 1] = colour.1;
                bufs1[(index * 4) as usize + 2] = colour.2;
                bufs1[(index * 4) as usize + 3] = (v * 255.0) as u8;
            });
        }
    }

    (glyphs_width + 20, glyphs_height, bufs1)
}

fn layout_paragraph<F, SF>(
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: Font,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = point(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        target.push(glyph);
    }
}
