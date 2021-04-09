import { readFileSync, createWriteStream } from 'fs';

import { newContext } from './lib/index.js';

const args = process.argv.slice(2);
const inputFile = args[0];
const contents = readFileSync(inputFile, 'utf-8');

const fontFile = args[1];
const fontContents = readFileSync(fontFile);

const ctx = await newContext();
ctx.registerFontData(fontContents);
const png = ctx.render(contents, 2);

const outputFile = createWriteStream('./out.png');
outputFile.write(png);
outputFile.end();

console.log(ctx.listFonts());