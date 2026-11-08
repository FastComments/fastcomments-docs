## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createQuestionResultBody | CreateQuestionResultBody | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResultResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'createQuestionResult उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "q_9876",
  answer: "Yes",
  score: 5,
  meta: [
    { key: "source", value: "survey" }
  ]
};

const result: CreateQuestionResultResponse = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]