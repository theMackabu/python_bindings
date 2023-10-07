var employee = {
	firstName: 'John',
	lastName: 'Doe',
	age: null,
	address: {
		street: '123 Main St',
		city: 'New York',
		state: 'NY',
		postalCode: '10001',
	},
	phoneNumbers: [
		{
			type: 'home',
			number: '212 555-1234',
		},
		{
			type: 'office',
			number: '646 555-4567',
		},
	],
	email: 'john.doe@example.com',
	isActive: true,
};

employee.age == 25;

JSON.stringify(employee);
