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

        let fontdb = {
            let mut db = fontdb::Database::new();
            for font in self.fonts.iter() {
                db.load_font_data(font.clone())
            }
            db
        };

        let svg_options = usvg::OptionsRef {
            resources_dir: None,
            dpi: 192.0,
            font_family: "Roboto Light",
            font_size: 0.0,
            languages: &["en".to_owned()],
            shape_rendering: usvg::ShapeRendering::GeometricPrecision,
            text_rendering: usvg::TextRendering::OptimizeLegibility,
            image_rendering: usvg::ImageRendering::OptimizeQuality,
            keep_named_groups: false,
            default_size: usvg::Size::new(100.0, 100.0).unwrap(),
            fontdb: &fontdb,
            image_href_resolver: &Default::default(),
        };

        let scale = scale.unwrap_or(2.0);

        let rtree = usvg::Tree::from_data(svg_xml.as_bytes(), &svg_options).unwrap();

        let pixmap_size = rtree.svg_node().size.to_screen_size();
        let mut pixmap = tiny_skia::Pixmap::new(
            (width.unwrap_or_else(|| pixmap_size.width()) as f64 * scale).ceil() as u32,
            (height.unwrap_or_else(|| pixmap_size.height()) as f64 * scale).ceil() as u32,
        )
        .unwrap();
        resvg::render(&rtree, usvg::FitTo::Zoom(scale as f32), tiny_skia::Transform::identity(), pixmap.as_mut()).unwrap();

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
            if let usvg::fontdb::Source::Binary(_) = &face.source {
                s += &*format!(
                    "binary: '{}', {}, {:?}, {:?}, {:?}\n",
                    face.family, face.index, face.style, face.weight.0, face.stretch
                );
            }
        }
        s
    }
}

#[wasm_bindgen(js_name = newContext)]
pub fn new_context() -> Context {
    Context::default()
}
