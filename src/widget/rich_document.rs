use crate::graphic::base::{Align, DEFAULT_FONT_PATH, RGBA, TextStyle};

/// 带样式的单个字符
#[derive(Clone, Debug, PartialEq)]
pub struct StyledChar {
    pub ch: char,
    pub style: TextStyle,
}

/// 字符级富文本文档（状态必须放在 Instance 中，layout 会重建控件树）
#[derive(Clone, Debug, PartialEq)]
pub struct RichDocument {
    pub chars: Vec<StyledChar>,
    pub caret: usize,
    pub sel_anchor: Option<usize>,
    pub current_style: TextStyle,
    pub scroll: f32,
    pub is_focus: bool,
    pub dragging: bool,
    pub fonts: Vec<String>,
}

impl Default for RichDocument {
    fn default() -> Self {
        Self {
            chars: Vec::new(),
            caret: 0,
            sel_anchor: None,
            current_style: TextStyle::default(),
            scroll: 0.0,
            is_focus: false,
            dragging: false,
            fonts: vec![DEFAULT_FONT_PATH.to_string()],
        }
    }
}

impl RichDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_plain(text: &str) -> Self {
        let style = TextStyle::default();
        let chars = text
            .chars()
            .map(|ch| StyledChar { ch, style })
            .collect::<Vec<_>>();
        let caret = chars.len();
        Self {
            chars,
            caret,
            ..Self::default()
        }
    }

    fn clamp_index(&self, i: usize) -> usize {
        i.min(self.chars.len())
    }

    pub fn selection(&self) -> Option<(usize, usize)> {
        let a = self.sel_anchor?;
        let len = self.chars.len();
        let lo = a.min(self.caret).min(len);
        let hi = a.max(self.caret).min(len);
        if lo == hi {
            None
        } else {
            Some((lo, hi))
        }
    }

    pub fn clear_selection(&mut self) {
        self.sel_anchor = None;
    }

    pub fn delete_selection(&mut self) -> bool {
        if let Some((lo, hi)) = self.selection() {
            self.chars.drain(lo..hi);
            self.caret = lo;
            self.sel_anchor = None;
            true
        } else {
            false
        }
    }

    pub fn insert(&mut self, c: char) {
        self.delete_selection();
        self.caret = self.clamp_index(self.caret);
        let style = self.current_style;
        self.chars.insert(self.caret, StyledChar { ch: c, style });
        self.caret += 1;
        self.sel_anchor = None;
    }

    pub fn delete_backward(&mut self) {
        if self.delete_selection() {
            return;
        }
        if self.caret > 0 {
            self.caret -= 1;
            self.chars.remove(self.caret);
        }
    }

    pub fn delete_forward(&mut self) {
        if self.delete_selection() {
            return;
        }
        if self.caret < self.chars.len() {
            self.chars.remove(self.caret);
        }
    }

    pub fn move_caret(&mut self, pos: usize) {
        self.caret = pos.min(self.chars.len());
        self.sel_anchor = None;
    }

    pub fn extend_selection(&mut self, pos: usize) {
        if self.sel_anchor.is_none() {
            self.sel_anchor = Some(self.caret);
        }
        self.caret = pos.min(self.chars.len());
    }

    pub fn apply_to_selection<F: Fn(&mut TextStyle)>(&mut self, f: F) {
        let mut style = self.display_style();
        f(&mut style);
        self.current_style = style;
        if let Some((lo, hi)) = self.selection() {
            let hi = hi.min(self.chars.len());
            let lo = lo.min(hi);
            for ch in &mut self.chars[lo..hi] {
                ch.style = style;
            }
        }
    }

    pub fn select_all(&mut self) {
        if self.chars.is_empty() {
            self.caret = 0;
            self.sel_anchor = None;
            return;
        }
        self.sel_anchor = Some(0);
        self.caret = self.chars.len();
    }

    pub fn display_style(&self) -> TextStyle {
        if let Some((lo, _)) = self.selection() {
            if let Some(ch) = self.chars.get(lo) {
                return ch.style;
            }
        }
        self.current_style
    }

    pub fn copy_selection(&self) -> Vec<StyledChar> {
        if let Some((lo, hi)) = self.selection() {
            self.chars[lo..hi].to_vec()
        } else {
            Vec::new()
        }
    }

    pub fn cut_selection(&mut self) -> Vec<StyledChar> {
        let copied = self.copy_selection();
        self.delete_selection();
        copied
    }

    pub fn paste(&mut self, clip: &[StyledChar]) {
        self.delete_selection();
        self.caret = self.clamp_index(self.caret);
        for (i, sc) in clip.iter().enumerate() {
            self.chars.insert(self.caret + i, sc.clone());
        }
        self.caret += clip.len();
        self.sel_anchor = None;
    }

    pub fn to_plain(&self) -> String {
        self.chars.iter().map(|c| c.ch).collect()
    }

    pub fn to_toml(&self) -> String {
        let global = self.current_style;
        let mut content = Vec::new();
        let mut i = 0;
        while i < self.chars.len() {
            let style = self.chars[i].style;
            let mut text = String::new();
            while i < self.chars.len() && self.chars[i].style == style {
                text.push(self.chars[i].ch);
                i += 1;
            }
            content.push(ContentDto {
                text: Some(text),
                image: None,
                style: style_diff(global, style),
            });
        }
        let file = FileDoc {
            format: FORMAT_MAGIC.to_string(),
            version: FORMAT_VERSION,
            caret: self.caret,
            fonts: self.fonts.clone(),
            style: StyleDto::from_full(global),
            content,
        };
        toml_encode(&file)
    }

    pub fn from_toml(text: &str) -> Option<Self> {
        let file = toml_decode(text)?;
        if file.format != FORMAT_MAGIC || file.version != FORMAT_VERSION {
            return None;
        }
        let mut fonts = file.fonts;
        if fonts.is_empty() {
            fonts.push(DEFAULT_FONT_PATH.to_string());
        }
        let font_max = (fonts.len().saturating_sub(1)) as u8;
        let global = file.style.to_style(TextStyle::default(), font_max);
        let mut chars = Vec::new();
        for block in &file.content {
            let Some(run) = &block.text else {
                continue;
            };
            let style = match &block.style {
                Some(dto) => dto.to_style(global, font_max),
                None => global,
            };
            for ch in run.chars() {
                chars.push(StyledChar { ch, style });
            }
        }
        let caret = file.caret.min(chars.len());
        Some(Self {
            chars,
            caret,
            current_style: global,
            fonts,
            ..Self::default()
        })
    }

    pub fn content_eq(&self, other: &Self) -> bool {
        self.chars == other.chars && self.fonts == other.fonts
    }
}

const FORMAT_MAGIC: &str = "lemogui-rich";
const FORMAT_VERSION: u32 = 1;

#[derive(Default)]
struct FileDoc {
    format: String,
    version: u32,
    caret: usize,
    fonts: Vec<String>,
    style: StyleDto,
    content: Vec<ContentDto>,
}

#[derive(Default, Clone)]
struct StyleDto {
    bold: Option<bool>,
    italic: Option<bool>,
    underline: Option<bool>,
    size: Option<f32>,
    color: Option<[f32; 4]>,
    align: Option<String>,
    font: Option<u8>,
}

#[derive(Default)]
struct ContentDto {
    text: Option<String>,
    image: Option<String>,
    style: Option<StyleDto>,
}

impl StyleDto {
    fn from_full(style: TextStyle) -> Self {
        Self {
            bold: Some(style.bold),
            italic: Some(style.italic),
            underline: Some(style.underline),
            size: Some(style.size),
            color: Some([
                style.color.0,
                style.color.1,
                style.color.2,
                style.color.3,
            ]),
            align: Some(align_to_str(style.align).to_string()),
            font: Some(style.font),
        }
    }

    fn to_style(&self, base: TextStyle, font_max: u8) -> TextStyle {
        TextStyle {
            bold: self.bold.unwrap_or(base.bold),
            italic: self.italic.unwrap_or(base.italic),
            underline: self.underline.unwrap_or(base.underline),
            size: self.size.unwrap_or(base.size),
            color: self
                .color
                .map(|c| RGBA(c[0], c[1], c[2], c[3]))
                .unwrap_or(base.color),
            align: self
                .align
                .as_deref()
                .map(align_from_str)
                .unwrap_or(base.align),
            font: clamp_font(self.font.unwrap_or(base.font), font_max),
        }
    }

    fn apply_kv(&mut self, key: &str, value: &str) -> Option<()> {
        match key {
            "bold" => self.bold = Some(parse_bool(value)?),
            "italic" => self.italic = Some(parse_bool(value)?),
            "underline" => self.underline = Some(parse_bool(value)?),
            "size" => self.size = Some(parse_f32(value)?),
            "color" => self.color = Some(parse_color(value)?),
            "align" => self.align = Some(parse_string(value)?),
            "font" => self.font = Some(parse_u32(value)? as u8),
            _ => {}
        }
        Some(())
    }
}

fn style_diff(global: TextStyle, run: TextStyle) -> Option<StyleDto> {
    if global == run {
        return None;
    }
    Some(StyleDto {
        bold: (run.bold != global.bold).then_some(run.bold),
        italic: (run.italic != global.italic).then_some(run.italic),
        underline: (run.underline != global.underline).then_some(run.underline),
        size: (run.size != global.size).then_some(run.size),
        color: (run.color != global.color).then_some([
            run.color.0,
            run.color.1,
            run.color.2,
            run.color.3,
        ]),
        align: (run.align != global.align).then(|| align_to_str(run.align).to_string()),
        font: (run.font != global.font).then_some(run.font),
    })
}

fn clamp_font(font: u8, font_max: u8) -> u8 {
    if font <= font_max {
        font
    } else {
        0
    }
}

fn align_to_str(align: Align) -> &'static str {
    match align {
        Align::Left => "left",
        Align::Center => "center",
        Align::Right => "right",
    }
}

fn align_from_str(s: &str) -> Align {
    match s {
        "center" => Align::Center,
        "right" => Align::Right,
        _ => Align::Left,
    }
}

fn toml_encode(file: &FileDoc) -> String {
    let mut out = String::new();
    out.push_str(&format!("format = {}\n", quote(&file.format)));
    out.push_str(&format!("version = {}\n", file.version));
    out.push_str(&format!("caret = {}\n", file.caret));
    out.push_str("fonts = [");
    for (i, font) in file.fonts.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&quote(font));
    }
    out.push_str("]\n\n[style]\n");
    write_style(&mut out, &file.style);
    for block in &file.content {
        out.push_str("\n[[content]]\n");
        if let Some(text) = &block.text {
            out.push_str("text = ");
            out.push_str(&quote(text));
            out.push('\n');
        }
        if let Some(image) = &block.image {
            out.push_str("image = ");
            out.push_str(&quote(image));
            out.push('\n');
        }
        if let Some(style) = &block.style {
            out.push_str("\n[content.style]\n");
            write_style(&mut out, style);
        }
    }
    out
}

fn write_style(out: &mut String, style: &StyleDto) {
    if let Some(v) = style.bold {
        out.push_str(&format!("bold = {}\n", v));
    }
    if let Some(v) = style.italic {
        out.push_str(&format!("italic = {}\n", v));
    }
    if let Some(v) = style.underline {
        out.push_str(&format!("underline = {}\n", v));
    }
    if let Some(v) = style.size {
        out.push_str(&format!("size = {}\n", v));
    }
    if let Some(c) = style.color {
        out.push_str(&format!("color = [{}, {}, {}, {}]\n", c[0], c[1], c[2], c[3]));
    }
    if let Some(v) = &style.align {
        out.push_str("align = ");
        out.push_str(&quote(v));
        out.push('\n');
    }
    if let Some(v) = style.font {
        out.push_str(&format!("font = {}\n", v));
    }
}

fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

enum TomlSection {
    Root,
    Style,
    Content,
    ContentStyle,
}

fn toml_decode(text: &str) -> Option<FileDoc> {
    let mut file = FileDoc::default();
    let mut section = TomlSection::Root;
    for raw in text.lines() {
        let line = strip_comment(raw).trim().to_string();
        if line.is_empty() {
            continue;
        }
        if line == "[style]" {
            section = TomlSection::Style;
            continue;
        }
        if line == "[[content]]" {
            file.content.push(ContentDto::default());
            section = TomlSection::Content;
            continue;
        }
        if line == "[content.style]" {
            let last = file.content.last_mut()?;
            if last.style.is_none() {
                last.style = Some(StyleDto::default());
            }
            section = TomlSection::ContentStyle;
            continue;
        }
        if line.starts_with('[') {
            continue;
        }
        let (key, value) = split_kv(&line)?;
        match section {
            TomlSection::Root => match key {
                "format" => file.format = parse_string(value)?,
                "version" => file.version = parse_u32(value)?,
                "caret" => file.caret = parse_u32(value)? as usize,
                "fonts" => file.fonts = parse_string_array(value)?,
                _ => {}
            },
            TomlSection::Style => {
                file.style.apply_kv(key, value)?;
            }
            TomlSection::Content => {
                let last = file.content.last_mut()?;
                match key {
                    "text" => last.text = Some(parse_string(value)?),
                    "image" => last.image = Some(parse_string(value)?),
                    "style" => last.style = Some(parse_inline_style(value)?),
                    _ => {}
                }
            }
            TomlSection::ContentStyle => {
                file.content.last_mut()?.style.as_mut()?.apply_kv(key, value)?;
            }
        }
    }
    Some(file)
}

fn strip_comment(line: &str) -> &str {
    let mut in_str = false;
    let mut escape = false;
    for (i, c) in line.char_indices() {
        if escape {
            escape = false;
            continue;
        }
        match c {
            '\\' if in_str => escape = true,
            '"' => in_str = !in_str,
            '#' if !in_str => return &line[..i],
            _ => {}
        }
    }
    line
}

fn split_kv(line: &str) -> Option<(&str, &str)> {
    let eq = line.find('=')?;
    let key = line[..eq].trim();
    let value = line[eq + 1..].trim();
    if key.is_empty() || value.is_empty() {
        return None;
    }
    Some((key, value))
}

fn parse_bool(v: &str) -> Option<bool> {
    match v {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn parse_u32(v: &str) -> Option<u32> {
    v.parse().ok()
}

fn parse_f32(v: &str) -> Option<f32> {
    v.parse().ok()
}

fn parse_string(v: &str) -> Option<String> {
    let v = v.trim();
    if v.len() < 2 || !v.starts_with('"') || !v.ends_with('"') {
        return Some(v.to_string());
    }
    let inner = &v[1..v.len() - 1];
    let mut out = String::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next()? {
                '\\' => out.push('\\'),
                '"' => out.push('"'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                c => out.push(c),
            }
        } else {
            out.push(c);
        }
    }
    Some(out)
}

fn parse_string_array(v: &str) -> Option<Vec<String>> {
    let v = v.trim();
    if !v.starts_with('[') || !v.ends_with(']') {
        return None;
    }
    let inner = v[1..v.len() - 1].trim();
    if inner.is_empty() {
        return Some(Vec::new());
    }
    let mut items = Vec::new();
    let mut cur = String::new();
    let mut in_str = false;
    let mut escape = false;
    for c in inner.chars() {
        if escape {
            cur.push(c);
            escape = false;
            continue;
        }
        match c {
            '\\' if in_str => {
                cur.push(c);
                escape = true;
            }
            '"' => {
                cur.push(c);
                in_str = !in_str;
            }
            ',' if !in_str => {
                items.push(parse_string(cur.trim())?);
                cur.clear();
            }
            c => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        items.push(parse_string(cur.trim())?);
    }
    Some(items)
}

fn parse_color(v: &str) -> Option<[f32; 4]> {
    let v = v.trim();
    if !v.starts_with('[') || !v.ends_with(']') {
        return None;
    }
    let parts: Vec<&str> = v[1..v.len() - 1].split(',').map(str::trim).collect();
    if parts.len() != 4 {
        return None;
    }
    Some([
        parts[0].parse().ok()?,
        parts[1].parse().ok()?,
        parts[2].parse().ok()?,
        parts[3].parse().ok()?,
    ])
}

fn parse_inline_style(v: &str) -> Option<StyleDto> {
    let v = v.trim();
    if !v.starts_with('{') || !v.ends_with('}') {
        return None;
    }
    let mut style = StyleDto::default();
    let inner = v[1..v.len() - 1].trim();
    if inner.is_empty() {
        return Some(style);
    }
    for part in inner.split(',') {
        let (k, val) = split_kv(part.trim())?;
        style.apply_kv(k, val)?;
    }
    Some(style)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_selection_does_not_panic() {
        let mut doc = RichDocument::from_plain("一二三四五六");
        assert_eq!(doc.chars.len(), 6);
        doc.sel_anchor = Some(0);
        doc.caret = 7;
        doc.apply_to_selection(|s| s.bold = true);
        doc.delete_selection();
        assert!(doc.chars.len() <= 6);
    }

    #[test]
    fn select_all_covers_document() {
        let mut doc = RichDocument::from_plain("在此输入富文本");
        doc.select_all();
        assert_eq!(doc.selection(), Some((0, 7)));
    }

    #[test]
    fn toml_roundtrip_keeps_text() {
        let mut doc = RichDocument::from_plain("ab\n文");
        doc.chars[0].style.bold = true;
        let back = RichDocument::from_toml(&doc.to_toml()).unwrap();
        assert_eq!(back.to_plain(), "ab\n文");
        assert!(back.chars[0].style.bold);
        assert!(!back.chars[1].style.bold);
        let dumped = doc.to_toml();
        assert!(dumped.contains("bold = true"));
    }

    #[test]
    fn toml_omits_style_when_same_as_global() {
        let doc = RichDocument::from_plain("普通正文");
        let toml = doc.to_toml();
        assert!(!toml.contains("[content.style]"));
        assert!(!toml.contains("style.bold"));
        let back = RichDocument::from_toml(&toml).unwrap();
        assert_eq!(back.to_plain(), "普通正文");
        assert_eq!(back.chars[0].style, back.current_style);
    }

    #[test]
    fn toml_missing_block_style_uses_global() {
        let src = r#"
format = "lemogui-rich"
version = 1

[style]
bold = true
size = 24.0
align = "center"

[[content]]
text = "Hi"
"#;
        let doc = RichDocument::from_toml(src).unwrap();
        assert_eq!(doc.to_plain(), "Hi");
        assert!(doc.current_style.bold);
        assert_eq!(doc.current_style.size, 24.0);
        assert_eq!(doc.current_style.align, Align::Center);
        assert!(doc.chars[0].style.bold);
        assert_eq!(doc.chars[0].style.size, 24.0);
    }

    #[test]
    fn toml_skips_image_blocks() {
        let src = r#"
format = "lemogui-rich"
version = 1

[[content]]
text = "A"

[[content]]
image = "img.png"

[[content]]
text = "B"
"#;
        let doc = RichDocument::from_toml(src).unwrap();
        assert_eq!(doc.to_plain(), "AB");
    }

    #[test]
    fn toml_keeps_font_path_and_align() {
        let mut doc = RichDocument::from_plain("X");
        doc.fonts.push("/tmp/extra.ttf".into());
        doc.chars[0].style.font = 1;
        doc.chars[0].style.align = Align::Center;
        let back = RichDocument::from_toml(&doc.to_toml()).unwrap();
        assert_eq!(back.fonts[1], "/tmp/extra.ttf");
        assert_eq!(back.chars[0].style.font, 1);
        assert_eq!(back.chars[0].style.align, Align::Center);
    }

    #[test]
    fn paste_inserts_at_caret() {
        let mut doc = RichDocument::from_plain("ac");
        doc.move_caret(1);
        doc.paste(&[StyledChar {
            ch: 'b',
            style: TextStyle::default(),
        }]);
        assert_eq!(doc.to_plain(), "abc");
    }
}
