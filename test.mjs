import { readFileSync, createWriteStream } from 'fs';

import { render } from './lib/index.js';

const args = process.argv.slice(2);
const inputFile = args[0];
let contents = readFileSync(inputFile, 'utf-8');

let png = await render(contents);

const outputFile = createWriteStream('./out.png');
outputFile.write(png);
outputFile.end();
