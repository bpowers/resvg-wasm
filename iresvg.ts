// Copyright 2021 Bobby Powers. All rights reserved.
// Use of this source code is governed by the Mozilla Public
// License, Version 2.0, that can be found in the LICENSE file.

export interface Context {
  free(): void;
  [Symbol.dispose](): void;
  registerFontData(font_data: Uint8Array): void;
  render(
    svg_xml: string,
    scale?: number | null,
    width?: number | null,
    height?: number | null,
  ): Uint8Array | undefined;
  listFonts(): string;
}
