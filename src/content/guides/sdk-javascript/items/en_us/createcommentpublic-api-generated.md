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
const tenantId: string = 'tenant_prod_42';
const urlId: string = 'article-2026-03-25-tech-deep-dive';
const broadcastId: string = 'live-broadcast-001';
const sessionId: string = 'sess_9f8e7d6a3b';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const commentData: CommentData = {
  content: 'Great reporting — appreciated the depth on performance tradeoffs.',
  authorName: 'Jordan M.',
  language: 'en-US',
  metadata: { client: 'web' }
};
const result: CreateCommentPublic200Response = await createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso);
[inline-code-end]
