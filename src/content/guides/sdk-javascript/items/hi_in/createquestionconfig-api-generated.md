## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createQuestionConfigBody | CreateQuestionConfigBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfigResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'createQuestionConfig उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "acme-corp-123";

  const customOption: QuestionConfigCustomOptionsInner = {
    key: "maxLength",
    value: 500,
  };

  const createQuestionConfigBody: CreateQuestionConfigBody = {
    name: "User Feedback",
    description: "Collect user feedback after purchase",
    enabled: true,
    customOptions: [customOption],
    // वैकल्पिक फ़ील्ड
    tags: ["feedback", "post-purchase"],
  };

  const response: CreateQuestionConfigResponse = await createQuestionConfig(
    tenantId,
    createQuestionConfigBody
  );

  console.log(response);
}
[inline-code-end]