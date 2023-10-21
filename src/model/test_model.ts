import knex from "knex";



interface TestArgs {
	ruleId: number;
	testId: number;
};


const otherOther = async () => {
	console.log("Test TEst");
};

const testQuery = async (args: TestArgs) => {
	const test = knex("rules").select('*').where(args);

	return test;
};


const otherFunction = async () => {
	const test = "test string";

	const queryRespons = testQuery({ruleId: 123213, testId: 45335});
}

