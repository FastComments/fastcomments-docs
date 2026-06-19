## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | Yes |  |

## Response

Returns: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResultResponse.ts)

## Example

[inline-code-attrs-start title = 'createQuestionResult Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_5821b2";
  const createQuestionResultBody: CreateQuestionResultBody = {
    questionId: "q_9f3a7",
    userId: "user_77",
    score: 4.5,
    feedback: "Answer was concise and addressed the core issue",
    // optional parameters demonstrated
    sessionId: "sess_a12f",
    meta: [{ key: "device", value: "iPhone 13 Pro" }],
    status: { code: 201, message: "Created" }
  };
  const result: CreateQuestionResultResponse = await createQuestionResult(tenantId, createQuestionResultBody);
  console.log(result);
})();
[inline-code-end]
