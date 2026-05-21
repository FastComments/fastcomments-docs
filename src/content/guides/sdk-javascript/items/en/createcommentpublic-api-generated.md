## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| commentData | CommentData | Yes |  |
| sessionId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'createCommentPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9812';
const urlId: string = 'prod-launch-20260520';
const broadcastId: string = 'broadcast_20260520_01';
const commentData: CommentData = {
  content: 'Amazing launch — congrats to the whole team!',
  authorName: 'Alex Morgan',
  mentions: [] as CommentUserMentionInfo[],
  hashtags: [] as CommentUserHashTagInfo[]
};
const sessionId: string | undefined = 'sess_9f7b3c';
const sso: string | undefined = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const result: CreateCommentPublic200Response = await createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso);
[inline-code-end]
