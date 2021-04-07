// Copyright 2021 Bobby Powers. All rights reserved.
// Use of this source code is governed by the Mozilla Public
// License, Version 2.0, that can be found in the LICENSE file.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(svg_xml: &str) -> Option<Vec<u8>> {
    let svg_options = usvg::Options::default();

    let rtree = usvg::Tree::from_data(svg_xml.as_bytes(), &svg_options).unwrap();

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).unwrap();

    Some(pixmap.encode_png().unwrap())
}
