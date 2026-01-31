## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteQuestionResult Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprise-tenant-62';
const questionNumber: number = 12;
const optionalSuffix: string | undefined = undefined; // optional piece, may be omitted
const id: string = `qres-${questionNumber}-${optionalSuffix ?? 'v1'}-c9f4b2`;
const result: FlagCommentPublic200Response = await deleteQuestionResult(tenantId, id);
console.log('Deleted question result:', result);
[inline-code-end]
