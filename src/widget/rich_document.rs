use std::rc::Rc;

use crate::graphic::base::{Align, DEFAULT_FONT_PATH, RGBA, TextStyle};

/// 带样式的单个字符
#[derive(Clone, Debug, PartialEq)]
pub struct StyledChar {
    pub ch: char,
    pub style: TextStyle,
}

/// 图片排版
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageDisplay {
    Inline,
    Block,
}

/// 图片来源：路径链接或内嵌字节
#[derive(Clone, Debug, PartialEq)]
pub enum ImageSource {
    Path(String),
    Embedded(Rc<[u8]>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RichImage {
    pub source: ImageSource,
    pub display: ImageDisplay,
    /// 显示宽度（像素）；None 表示按原图（不超过行宽）
    pub width: Option<f32>,
}

/// 文档原子：一个字符或一张图
#[derive(Clone, Debug, PartialEq)]
pub enum RichAtom {
    Char(StyledChar),
    Image(RichImage),
}

/// 富文本文档（状态必须放在 Instance 中，layout 会重建控件树）
#[derive(Clone, Debug, PartialEq)]
pub struct RichDocument {
    pub atoms: Vec<RichAtom>,
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
            atoms: Vec::new(),
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

    pub fn len(&self) -> usize {
        self.atoms.len()
    }

    pub fn from_plain(text: &str) -> Self {
        let style = TextStyle::default();
        let atoms = text
            .chars()
            .map(|ch| RichAtom::Char(StyledChar { ch, style }))
            .collect::<Vec<_>>();
        let caret = atoms.len();
        Self {
            atoms,
            caret,
            ..Self::default()
        }
    }

    fn clamp_index(&self, i: usize) -> usize {
        i.min(self.atoms.len())
    }

    pub fn selection(&self) -> Option<(usize, usize)> {
        let a = self.sel_anchor?;
        let len = self.atoms.len();
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
            self.atoms.drain(lo..hi);
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
        self.atoms
            .insert(self.caret, RichAtom::Char(StyledChar { ch: c, style }));
        self.caret += 1;
        self.sel_anchor = None;
    }

    pub fn insert_image(&mut self, source: ImageSource, display: ImageDisplay) {
        self.delete_selection();
        self.caret = self.clamp_index(self.caret);
        self.atoms.insert(
            self.caret,
            RichAtom::Image(RichImage {
                source,
                display,
                width: None,
            }),
        );
        self.caret += 1;
        self.sel_anchor = None;
    }

    pub fn set_image_width(&mut self, index: usize, width: f32) {
        if let Some(RichAtom::Image(img)) = self.atoms.get_mut(index) {
            img.width = Some(width.max(24.0));
        }
    }

    pub fn delete_backward(&mut self) {
        if self.delete_selection() {
            return;
        }
        if self.caret > 0 {
            self.caret -= 1;
            self.atoms.remove(self.caret);
        }
    }

    pub fn delete_forward(&mut self) {
        if self.delete_selection() {
            return;
        }
        if self.caret < self.atoms.len() {
            self.atoms.remove(self.caret);
        }
    }

    pub fn move_caret(&mut self, pos: usize) {
        self.caret = pos.min(self.atoms.len());
        self.sel_anchor = None;
    }

    pub fn extend_selection(&mut self, pos: usize) {
        if self.sel_anchor.is_none() {
            self.sel_anchor = Some(self.caret);
        }
        self.caret = pos.min(self.atoms.len());
    }

    pub fn apply_to_selection<F: Fn(&mut TextStyle)>(&mut self, f: F) {
        let mut style = self.display_style();
        f(&mut style);
        self.current_style = style;
        if let Some((lo, hi)) = self.selection() {
            let hi = hi.min(self.atoms.len());
            let lo = lo.min(hi);
            for atom in &mut self.atoms[lo..hi] {
                if let RichAtom::Char(ch) = atom {
                    ch.style = style;
                }
            }
        }
    }

    pub fn select_all(&mut self) {
        if self.atoms.is_empty() {
            self.caret = 0;
            self.sel_anchor = None;
            return;
        }
        self.sel_anchor = Some(0);
        self.caret = self.atoms.len();
    }

    pub fn display_style(&self) -> TextStyle {
        if let Some((lo, hi)) = self.selection() {
            for atom in &self.atoms[lo..hi.min(self.atoms.len())] {
                if let RichAtom::Char(ch) = atom {
                    return ch.style;
                }
            }
        }
        self.current_style
    }

    pub fn copy_selection(&self) -> Vec<RichAtom> {
        if let Some((lo, hi)) = self.selection() {
            self.atoms[lo..hi].to_vec()
        } else {
            Vec::new()
        }
    }

    pub fn cut_selection(&mut self) -> Vec<RichAtom> {
        let copied = self.copy_selection();
        self.delete_selection();
        copied
    }

    pub fn paste(&mut self, clip: &[RichAtom]) {
        self.delete_selection();
        self.caret = self.clamp_index(self.caret);
        for (i, atom) in clip.iter().enumerate() {
            self.atoms.insert(self.caret + i, atom.clone());
        }
        self.caret += clip.len();
        self.sel_anchor = None;
    }

    pub fn to_plain(&self) -> String {
        self.atoms
            .iter()
            .filter_map(|a| match a {
                RichAtom::Char(c) => Some(c.ch),
                RichAtom::Image(_) => None,
            })
            .collect()
    }

    pub fn to_toml(&self) -> String {
        let global = self.current_style;
        let mut content = Vec::new();
        let mut i = 0;
        while i < self.atoms.len() {
            match &self.atoms[i] {
                RichAtom::Image(img) => {
                    content.push(image_to_dto(img));
                    i += 1;
                }
                RichAtom::Char(sc) => {
                    let style = sc.style;
                    let mut text = String::new();
                    while i < self.atoms.len() {
                        match &self.atoms[i] {
                            RichAtom::Char(c) if c.style == style => {
                                text.push(c.ch);
                                i += 1;
                            }
                            _ => break,
                        }
                    }
                    content.push(ContentDto {
                        text: Some(text),
                        style: style_diff(global, style),
                        ..ContentDto::default()
                    });
                }
            }
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
        let mut atoms = Vec::new();
        for block in &file.content {
            if let Some(run) = &block.text {
                let style = match &block.style {
                    Some(dto) => dto.to_style(global, font_max),
                    None => global,
                };
                for ch in run.chars() {
                    atoms.push(RichAtom::Char(StyledChar { ch, style }));
                }
                continue;
            }
            if let Some(img) = image_from_dto(block) {
                atoms.push(RichAtom::Image(img));
            }
        }
        let caret = file.caret.min(atoms.len());
        Some(Self {
            atoms,
            caret,
            current_style: global,
            fonts,
            ..Self::default()
        })
    }

    pub fn content_eq(&self, other: &Self) -> bool {
        self.atoms == other.atoms && self.fonts == other.fonts
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
    image_b64: Option<String>,
    display: Option<String>,
    width: Option<f32>,
    style: Option<StyleDto>,
}

fn image_to_dto(img: &RichImage) -> ContentDto {
    let display = match img.display {
        ImageDisplay::Inline => Some("inline".to_string()),
        ImageDisplay::Block => None,
    };
    let width = img.width;
    match &img.source {
        ImageSource::Path(path) => ContentDto {
            image: Some(path.clone()),
            display,
            width,
            ..ContentDto::default()
        },
        ImageSource::Embedded(bytes) => ContentDto {
            image_b64: Some(b64_encode(bytes)),
            display,
            width,
            ..ContentDto::default()
        },
    }
}

fn image_from_dto(block: &ContentDto) -> Option<RichImage> {
    let display = match block.display.as_deref() {
        Some("inline") => ImageDisplay::Inline,
        _ => ImageDisplay::Block,
    };
    let width = block.width.filter(|w| *w > 0.0);
    if let Some(b64) = &block.image_b64 {
        let payload = strip_data_uri(b64).unwrap_or(b64.as_str());
        let bytes = b64_decode(payload)?;
        return Some(RichImage {
            source: ImageSource::Embedded(Rc::from(bytes)),
            display,
            width,
        });
    }
    let img = block.image.as_ref()?;
    if let Some(payload) = strip_data_uri(img) {
        let bytes = b64_decode(payload)?;
        return Some(RichImage {
            source: ImageSource::Embedded(Rc::from(bytes)),
            display,
            width,
        });
    }
    Some(RichImage {
        source: ImageSource::Path(img.clone()),
        display,
        width,
    })
}

fn strip_data_uri(s: &str) -> Option<&str> {
    let s = s.trim();
    let rest = s.strip_prefix("data:")?;
    let idx = rest.find("base64,")?;
    Some(&rest[idx + 7..])
}

const B64_TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn b64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = data.get(i + 1).copied();
        let b2 = data.get(i + 2).copied();
        out.push(B64_TABLE[(b0 >> 2) as usize] as char);
        out.push(B64_TABLE[(((b0 & 3) << 4) | (b1.unwrap_or(0) >> 4)) as usize] as char);
        if b1.is_none() {
            out.push('=');
            out.push('=');
        } else {
            out.push(
                B64_TABLE[(((b1.unwrap() & 0xf) << 2) | (b2.unwrap_or(0) >> 6)) as usize] as char,
            );
            if b2.is_none() {
                out.push('=');
            } else {
                out.push(B64_TABLE[(b2.unwrap() & 0x3f) as usize] as char);
            }
        }
        i += 3;
    }
    out
}

fn b64_val(c: u8) -> Option<u8> {
    match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn b64_decode(s: &str) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let mut acc: u32 = 0;
    let mut n = 0;
    for c in s.bytes() {
        if c.is_ascii_whitespace() {
            continue;
        }
        if c == b'=' {
            break;
        }
        let v = b64_val(c)?;
        acc = (acc << 6) | u32::from(v);
        n += 1;
        if n == 4 {
            buf.push((acc >> 16) as u8);
            buf.push((acc >> 8) as u8);
            buf.push(acc as u8);
            acc = 0;
            n = 0;
        }
    }
    match n {
        0 => {}
        2 => buf.push((acc >> 4) as u8),
        3 => {
            buf.push((acc >> 10) as u8);
            buf.push((acc >> 2) as u8);
        }
        _ => return None,
    }
    Some(buf)
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
        if let Some(b64) = &block.image_b64 {
            out.push_str("image_b64 = ");
            out.push_str(&quote(b64));
            out.push('\n');
        }
        if let Some(display) = &block.display {
            out.push_str("display = ");
            out.push_str(&quote(display));
            out.push('\n');
        }
        if let Some(w) = block.width {
            out.push_str(&format!("width = {}\n", w));
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
                    "image_b64" => last.image_b64 = Some(parse_string(value)?),
                    "display" => last.display = Some(parse_string(value)?),
                    "width" => last.width = Some(parse_f32(value)?),
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
        assert_eq!(doc.len(), 6);
        doc.sel_anchor = Some(0);
        doc.caret = 7;
        doc.apply_to_selection(|s| s.bold = true);
        doc.delete_selection();
        assert!(doc.len() <= 6);
    }

    #[test]
    fn select_all_covers_document() {
        let mut doc = RichDocument::from_plain("在此输入富文本");
        doc.select_all();
        assert_eq!(doc.selection(), Some((0, 7)));
    }

    fn atom_char(doc: &RichDocument, i: usize) -> &StyledChar {
        match &doc.atoms[i] {
            RichAtom::Char(c) => c,
            RichAtom::Image(_) => panic!("expected char"),
        }
    }

    #[test]
    fn toml_roundtrip_keeps_text() {
        let mut doc = RichDocument::from_plain("ab\n文");
        if let RichAtom::Char(c) = &mut doc.atoms[0] {
            c.style.bold = true;
        }
        let back = RichDocument::from_toml(&doc.to_toml()).unwrap();
        assert_eq!(back.to_plain(), "ab\n文");
        assert!(atom_char(&back, 0).style.bold);
        assert!(!atom_char(&back, 1).style.bold);
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
        assert_eq!(atom_char(&back, 0).style, back.current_style);
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
        assert!(atom_char(&doc, 0).style.bold);
        assert_eq!(atom_char(&doc, 0).style.size, 24.0);
    }

    #[test]
    fn toml_keeps_image_path() {
        let src = r#"
format = "lemogui-rich"
version = 1

[[content]]
text = "A"

[[content]]
image = "img.png"
display = "inline"
width = 200

[[content]]
text = "B"
"#;
        let doc = RichDocument::from_toml(src).unwrap();
        assert_eq!(doc.to_plain(), "AB");
        assert_eq!(doc.len(), 3);
        match &doc.atoms[1] {
            RichAtom::Image(img) => {
                assert_eq!(img.source, ImageSource::Path("img.png".into()));
                assert_eq!(img.display, ImageDisplay::Inline);
                assert_eq!(img.width, Some(200.0));
            }
            _ => panic!("expected image"),
        }
        let dumped = doc.to_toml();
        assert!(dumped.contains("image = \"img.png\""));
        assert!(dumped.contains("display = \"inline\""));
        assert!(dumped.contains("width = 200"));
    }

    #[test]
    fn toml_roundtrip_image_b64() {
        let bytes: Rc<[u8]> = Rc::from(&b"png-bytes"[..]);
        let mut doc = RichDocument::from_plain("A");
        doc.insert_image(ImageSource::Embedded(Rc::clone(&bytes)), ImageDisplay::Block);
        doc.insert('B');
        let dumped = doc.to_toml();
        assert!(dumped.contains("image_b64 ="));
        assert!(!dumped.contains("display ="));
        let back = RichDocument::from_toml(&dumped).unwrap();
        assert_eq!(back.to_plain(), "AB");
        match &back.atoms[1] {
            RichAtom::Image(img) => match &img.source {
                ImageSource::Embedded(b) => assert_eq!(&b[..], b"png-bytes"),
                _ => panic!("expected embedded"),
            },
            _ => panic!("expected image"),
        }
    }

    #[test]
    fn toml_data_uri_becomes_embedded() {
        let payload = b64_encode(b"xyz");
        let src = format!(
            r#"
format = "lemogui-rich"
version = 1

[[content]]
image = "data:image/png;base64,{payload}"
"#
        );
        let doc = RichDocument::from_toml(&src).unwrap();
        match &doc.atoms[0] {
            RichAtom::Image(img) => match &img.source {
                ImageSource::Embedded(b) => assert_eq!(&b[..], b"xyz"),
                _ => panic!("expected embedded"),
            },
            _ => panic!("expected image"),
        }
    }

    #[test]
    fn toml_roundtrip_image_width() {
        let mut doc = RichDocument::from_plain("");
        doc.insert_image(ImageSource::Path("a.png".into()), ImageDisplay::Block);
        doc.set_image_width(0, 180.0);
        let dumped = doc.to_toml();
        assert!(dumped.contains("width = 180"));
        let back = RichDocument::from_toml(&dumped).unwrap();
        match &back.atoms[0] {
            RichAtom::Image(img) => assert_eq!(img.width, Some(180.0)),
            _ => panic!("expected image"),
        }
    }

    #[test]
    fn toml_bad_b64_is_skipped() {
        let src = r#"
format = "lemogui-rich"
version = 1

[[content]]
text = "A"

[[content]]
image_b64 = "!!!!"

[[content]]
text = "B"
"#;
        let doc = RichDocument::from_toml(src).unwrap();
        assert_eq!(doc.to_plain(), "AB");
        assert_eq!(doc.len(), 2);
    }

    #[test]
    fn b64_roundtrip() {
        let src = b"hello\x00\xff world";
        assert_eq!(b64_decode(&b64_encode(src)).unwrap(), src);
    }

    #[test]
    fn toml_keeps_font_path_and_align() {
        let mut doc = RichDocument::from_plain("X");
        doc.fonts.push("/tmp/extra.ttf".into());
        if let RichAtom::Char(c) = &mut doc.atoms[0] {
            c.style.font = 1;
            c.style.align = Align::Center;
        }
        let back = RichDocument::from_toml(&doc.to_toml()).unwrap();
        assert_eq!(back.fonts[1], "/tmp/extra.ttf");
        assert_eq!(atom_char(&back, 0).style.font, 1);
        assert_eq!(atom_char(&back, 0).style.align, Align::Center);
    }

    #[test]
    fn paste_inserts_at_caret() {
        let mut doc = RichDocument::from_plain("ac");
        doc.move_caret(1);
        doc.paste(&[RichAtom::Char(StyledChar {
            ch: 'b',
            style: TextStyle::default(),
        })]);
        assert_eq!(doc.to_plain(), "abc");
    }
}
