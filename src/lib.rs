// Copyright 2021 Bobby Powers. All rights reserved.
// Use of this source code is governed by the Mozilla Public
// License, Version 2.0, that can be found in the LICENSE file.

use std::sync::Arc;

use resvg::tiny_skia;
use resvg::usvg;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default, Clone)]
pub struct Context {
    fonts: Vec<Vec<u8>>,
}

#[wasm_bindgen]
impl Context {
    #[wasm_bindgen(js_name = registerFontData)]
    pub fn register_font_data(&mut self, font_data: &[u8]) {
        self.fonts.push(font_data.to_vec());
    }

    #[wasm_bindgen]
    pub fn render(
        &self,
        svg_xml: &str,
        scale: Option<f64>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Option<Vec<u8>> {
        let mut fontdb = usvg::fontdb::Database::new();
        for font in &self.fonts {
            fontdb.load_font_data(font.clone());
        }

        let svg_options = usvg::Options {
            dpi: 192.0,
            font_family: "Roboto Light".to_owned(),
            shape_rendering: usvg::ShapeRendering::GeometricPrecision,
            text_rendering: usvg::TextRendering::OptimizeLegibility,
            image_rendering: usvg::ImageRendering::OptimizeQuality,
            fontdb: Arc::new(fontdb),
            ..usvg::Options::default()
        };

        let scale = scale.unwrap_or(2.0);

        let tree = usvg::Tree::from_str(svg_xml, &svg_options).ok()?;

        let size = tree.size().to_int_size();
        let scaled_width = (width.unwrap_or(size.width()) as f64 * scale).ceil() as u32;
        let scaled_height = (height.unwrap_or(size.height()) as f64 * scale).ceil() as u32;

        let mut pixmap = tiny_skia::Pixmap::new(scaled_width, scaled_height)?;

        let transform = tiny_skia::Transform::from_scale(scale as f32, scale as f32);
        resvg::render(&tree, transform, &mut pixmap.as_mut());

        Some(pixmap.encode_png().ok()?)
    }

    #[wasm_bindgen(js_name = listFonts)]
    pub fn list_fonts(&self) -> String {
        let mut fontdb = usvg::fontdb::Database::new();

        let mut s = "fonts:\n".to_owned();
        for font in &self.fonts {
            s += &format!("loading buf len {}\n", font.len());
            fontdb.load_font_data(font.clone());
        }

        for face in fontdb.faces() {
            s += &format!(
                "binary: '{}', {}, {:?}, {:?}, {:?}\n",
                face.families[0].0,
                face.index,
                face.style,
                face.weight.0,
                face.stretch
            );
        }
        s
    }
}

#[wasm_bindgen(js_name = newContext)]
pub fn new_context() -> Context {
    Context::default()
}
