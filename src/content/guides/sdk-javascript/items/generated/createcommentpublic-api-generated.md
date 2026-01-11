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
const tenantId: string = 'tenant_8b3f2a';
const urlId: string = 'articles/2025/11/solar-eclipse-grid-impact';
const broadcastId: string = 'broadcast_4721';
const commentData: CommentData = {
  body: 'Thanks for the in-depth coverage — wondering how local utilities are preparing for the eclipse day demand spike.',
  parentId: null,
  authorName: 'Jordan Meyers',
  authorEmail: 'j.meyers@newsdaily.com',
  mentions: [{ userId: 'user_2468', displayName: 'Alex Rivera' }],
  hashtags: [{ tag: 'solar-eclipse' }],
  metadata: { device: 'desktop', browser: 'Chrome/120' }
};
const sessionId: string = 'sess_9f3a2b7c';
const sso: string = 'sso.jwt.token.eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const result: CreateCommentPublic200Response = await createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso);
[inline-code-end]
