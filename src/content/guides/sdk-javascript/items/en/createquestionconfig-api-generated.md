## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Yes |  |

## Response

Returns: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfigResponse.ts)

## Example

[inline-code-attrs-start title = 'createQuestionConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    // optional field
    tags: ["feedback", "post-purchase"],
  };

  const response: CreateQuestionConfigResponse = await createQuestionConfig(
    tenantId,
    createQuestionConfigBody
  );

  console.log(response);
}
[inline-code-end]
