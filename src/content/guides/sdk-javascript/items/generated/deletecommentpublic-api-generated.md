## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteCommentPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42ac9b0f';
const commentId: string = 'cmt_5f8b7d3a9c';
const broadcastId: string = 'broadcast_2025-11-22T12:00:00Z';
const editKey: string = 'edk-3b9f7a2c';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.signature';
const result: DeleteCommentPublic200Response = await deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso);
console.log(result);
[inline-code-end]
