## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voteId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteCommentVote Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4b2f9';
const commentId: string = 'cmt_921f3';
const voteId: string = 'vote_3a8b1';
const urlId: string = 'https://news.site/articles/2026/05/20/launch';
const broadcastId: string = 'broadcast_20260520_01';
const editKey: string = 'editkey_7d2c';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature';
const result: DeleteCommentVote200Response = await deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, editKey, sso);
[inline-code-end]
