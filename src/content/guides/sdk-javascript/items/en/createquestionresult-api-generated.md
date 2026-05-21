## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | Yes |  |

## Response

Returns: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## Example

[inline-code-attrs-start title = 'createQuestionResult Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3a2b';
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: 'q_92a1',
  respondentId: 'user_71c',
  answers: [{ subQuestionId: 'sq1', value: 'Yes' }],
  meta: [{ key: 'source', value: 'web' }] // optional metadata
};
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]
