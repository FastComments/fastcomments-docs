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
const tenantId: string = 'tenant_acme_corp_2026';
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: 'q_98765',
  userId: 'user_12345',
  score: 4,
  comment: 'Clear explanation, helped resolve issue.',
  meta: [{ key: 'source', value: 'web' }] // optional metadata
};
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]
