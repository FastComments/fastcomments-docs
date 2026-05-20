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
const tenantId: string = 'tenant_98765';
const commentId: string = 'cmt_4521';
const broadcastId: string = 'broadcast_live_01';
const editKey: string = 'edit_3a2b'; // optional
const sso: string = 'sso_token_abcd1234'; // optional
const result: DeleteCommentPublic200Response = await deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso);
[inline-code-end]
