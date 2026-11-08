## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'updateQuestionConfig उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const questionId: string = "question-42";

  const updateBody: UpdateQuestionConfigBody = {
    title: "Revised FAQ Question"
    // isActive, customOptions, आदि वैकल्पिक हैं और छोड़ दिए गए हैं
  };

  const response: APIEmptyResponse = await updateQuestionConfig(tenantId, questionId, updateBody);
  console.log(response);
})();
[inline-code-end]