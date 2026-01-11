## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentText200Response.ts)

## Example

[inline-code-attrs-start title = 'setCommentText Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_981c';
const commentId: string = 'cmt-2048';
const broadcastId: string = 'broadcast-nyc-2025-11-22';
const commentTextUpdateRequest: CommentTextUpdateRequest = {
  text: 'Thanks for the update — I think @maria raises a good point. See #product-roadmap',
  mentions: [{ userId: 'user_maria', displayName: 'María López' }],
  hashtags: [{ tag: 'product-roadmap' }]
};
const editKey: string = 'editkey_7f3a';
const sso: string = 'sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const result: SetCommentText200Response = await setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso);
[inline-code-end]
