## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## Example

[inline-code-attrs-start title = 'getQuestionResult Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantSuffix: string | undefined = undefined;
const tenantId: string = `fastcomments-company-42${tenantSuffix ? `-${tenantSuffix}` : ''}`;
const id: string = 'question_20260109_7f3b';
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, id);
[inline-code-end]
