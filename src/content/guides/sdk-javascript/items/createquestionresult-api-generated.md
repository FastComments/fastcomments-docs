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
const tenantId: string = "acme-tenant-001";
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "q-2026-01-satisfaction",
  commenterId: "commenter-4512",
  answers: [{ subQuestionId: "sq-1", value: 4 }, { subQuestionId: "sq-2", value: "yes" }],
  meta: [{ key: "page", value: "/product/sku-1234" }]
};
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]
