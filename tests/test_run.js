const student = {
	name: 'John Doe',
	age: 20,
	scores: {
		math: 90,
		science: 95,
		english: 85,
	},
};

console.log('student before processing:');
console.log(JSON.stringify(student));

student.age += 5;
student.scores.math += 10;

let totalScores = 0;
let scoreCount = 0;

for (let subject in student.scores) {
	totalScores += student.scores[subject];
	scoreCount++;
}

student.averageScore = (totalScores / scoreCount).toFixed(2);

console.log('student after processing:');
console.log(JSON.stringify(student));
