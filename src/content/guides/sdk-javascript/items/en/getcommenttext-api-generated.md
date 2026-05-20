## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentText200Response.ts)

## Example

[inline-code-attrs-start title = 'getCommentText Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-01';
const commentId: string = 'cmt_987654321';
const editKey: string = 'edk_abc123def';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';

const resultWithOptional: GetCommentText200Response = await getCommentText(tenantId, commentId, editKey, sso);
const resultRequiredOnly: GetCommentText200Response = await getCommentText(tenantId, commentId);
[inline-code-end]
