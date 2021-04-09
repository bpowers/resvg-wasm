// Copyright 2021 Bobby Powers. All rights reserved.
// Use of this source code is governed by the Mozilla Public
// License, Version 2.0, that can be found in the LICENSE file.

export interface Context {
  free(): void;
  /**
   * @param {Uint8Array} font_data
   */
  registerFontData(font_data: Uint8Array): void;
  /**
   * @param {string} svg_xml
   * @param {number | undefined} scale
   * @param {number | undefined} width
   * @param {number | undefined} height
   * @returns {Uint8Array | undefined}
   */
  render(
    svg_xml: string,
    scale?: number,
    width?: number,
    height?: number
  ): Uint8Array | undefined;
}
