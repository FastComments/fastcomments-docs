## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | No |  |
| userId | string | No |  |
| startDate | string | No |  |
| questionId | string | No |  |
| questionIds | string | No |  |
| skip | number | No |  |

## Response

Returns: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## Example

[inline-code-attrs-start title = 'getQuestionResults Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const urlId: string = 'article-2025-11-07';
const userId: string = 'user_7f3d';
const startDate: string = '2025-12-01T00:00:00Z';
const questionIds: string = 'q1,q2,q3';
const skip: number = 20;
const result: GetQuestionResults200Response = await getQuestionResults(tenantId, urlId, userId, startDate, undefined, questionIds, skip);
[inline-code-end]
