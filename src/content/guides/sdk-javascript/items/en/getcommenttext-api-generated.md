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
const tenantId: string = 'acme-corp-001';
const commentId: string = 'cmt_9f8b7a3e';
const editKey: string | undefined = 'edk_a1b2c3d4e5f6';
const sso: string | undefined = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.token.signature';

const commentText: GetCommentText200Response = await getCommentText(tenantId, commentId, editKey, sso);
[inline-code-end]
