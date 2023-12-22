const fs = require('fs');

const data = fs.readFileSync('input.txt', 'utf8');
const lines = data.split(/\r?\n/); // Splits the file content by new line


var trees = lines.map((line) => {
	return line.split('').map((c) => parseInt(c))
});

trees.pop()

var visible = 0 

trees.forEach((line, i) => {
	line.forEach((tree, j) => {
		if (i === 0 || i === trees.length - 1 || j === 0 || j === line.length - 1) {
			visible++
			return
		}
		
		if (line.slice(0, j+1).filter((e) => e >= tree).length === 1) {
			console.log('visible i ',i,' j ', j, ' value ', tree)
			visible++
			return
		}
		if (line.slice(j, line.length).filter((e) => e >= tree).length === 1 ) {
			console.log('visible i ',i,' j ', j, ' value ', tree)
			visible++
			return
		}
		if (trees.slice(0, i+1).map((row) => row[j]).filter((e) => e >= tree).length === 1) {
			console.log('visible i ',i,' j ', j, ' value ', tree)
			visible++
			return
		}
		if (trees.slice(i, trees.length).map((row) => row[j]).filter((e) => e >= tree).length === 1) {
			console.log('visible i ',i,' j ', j, ' value ', tree)
			visible++
			return
		}
	})
});

console.log(visible)
