// Copyright 2021 Bobby Powers. All rights reserved.
// Use of this source code is governed by the Mozilla Public
// License, Version 2.0, that can be found in the LICENSE file.

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
        &mut self,
        svg_xml: &str,
        scale: Option<f64>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Option<Vec<u8>> {
        let mut svg_options = usvg::Options::default();

        for font in self.fonts.iter() {
            svg_options.fontdb.load_font_data(font.clone())
        }

        let scale = scale.unwrap_or(2.0);

        svg_options.dpi = 192.0;
        svg_options.shape_rendering = usvg::ShapeRendering::GeometricPrecision;
        svg_options.text_rendering = usvg::TextRendering::OptimizeLegibility;
        svg_options.image_rendering = usvg::ImageRendering::OptimizeQuality;
        svg_options.font_family = "Roboto Light".to_owned();
        svg_options.languages = vec!["en".to_owned()];

        let rtree = usvg::Tree::from_data(svg_xml.as_bytes(), &svg_options).unwrap();

        let pixmap_size = rtree.svg_node().size.to_screen_size();
        let mut pixmap = tiny_skia::Pixmap::new(
            (width.unwrap_or_else(|| pixmap_size.width()) as f64 * scale).ceil() as u32,
            (height.unwrap_or_else(|| pixmap_size.height()) as f64 * scale).ceil() as u32,
        )
        .unwrap();
        resvg::render(&rtree, usvg::FitTo::Zoom(scale as f32), pixmap.as_mut()).unwrap();

        Some(pixmap.encode_png().unwrap())
    }

    #[wasm_bindgen(js_name = listFonts)]
    pub fn list_fonts(&self) -> String {
        let mut svg_options = usvg::Options::default();

        let mut s = "fonts:\n".to_owned();
        for font in self.fonts.iter() {
            s += &*format!("loading buf len {}\n", font.len());
            svg_options.fontdb.load_font_data(font.clone())
        }

        for face in svg_options.fontdb.faces() {
            let usvg::fontdb::Source::Binary(_) = &*face.source;
            s += &*format!(
                "binary: '{}', {}, {:?}, {:?}, {:?}\n",
                face.family, face.index, face.style, face.weight.0, face.stretch
            );
        }
        s
    }
}

#[wasm_bindgen(js_name = newContext)]
pub fn new_context() -> Context {
    Context::default()
}
