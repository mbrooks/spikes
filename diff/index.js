#!/usr/bin/env node

require('colors');
const Diff = require('diff');

const string1 = `
Tennis racket
Bicycle pump
USB drive
LED lightbulb
French press
Wok
Watercolor paint set
Pocket knife
Yoga mat
Electric kettle
`;


const string2 = `
Tennis racket
LED lightbulb
French press
Wok
`;

const oldFileName = 'one';
const newFileName = 'two';
const diff = Diff.createTwoFilesPatch(oldFileName, newFileName, string1, string2);

diff.split("\n").forEach((part) => {
	if (part.startsWith('+')) {
		const color = 'green';
		process.stderr.write(part[color] + "\n");
	} else if (part.startsWith('-')) {
		const color = 'red';
		process.stderr.write(part[color] + "\n");
	} else {
		const color = 'grey';
		process.stderr.write(part[color] + "\n");
	}
  });
  
  console.log();