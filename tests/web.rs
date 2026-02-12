#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use resvg_wasm::{new_context, Context};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_new_context() {
    let _ctx = new_context();
}

#[wasm_bindgen_test]
fn test_render_simple_svg() {
    let ctx = new_context();
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
        <rect width="100" height="100" fill="red"/>
    </svg>"#;
    let result = ctx.render(svg, None, None, None);
    assert!(result.is_some());
    let png_data = result.unwrap();
    assert!(png_data.len() > 0);
    // PNG magic bytes
    assert_eq!(&png_data[..4], &[0x89, 0x50, 0x4E, 0x47]);
}

#[wasm_bindgen_test]
fn test_render_with_scale() {
    let ctx = new_context();
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="50" height="50">
        <circle cx="25" cy="25" r="20" fill="blue"/>
    </svg>"#;
    let result = ctx.render(svg, Some(1.0), None, None);
    assert!(result.is_some());
}

#[wasm_bindgen_test]
fn test_render_with_explicit_dimensions() {
    let ctx = new_context();
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
        <rect width="100" height="100" fill="green"/>
    </svg>"#;
    let result = ctx.render(svg, Some(1.0), Some(200), Some(200));
    assert!(result.is_some());
}

#[wasm_bindgen_test]
fn test_render_invalid_svg_returns_none() {
    let ctx = new_context();
    let result = ctx.render("not valid svg", None, None, None);
    assert!(result.is_none());
}

#[wasm_bindgen_test]
fn test_list_fonts_empty() {
    let ctx = new_context();
    let result = ctx.list_fonts();
    assert!(result.starts_with("fonts:\n"));
}
