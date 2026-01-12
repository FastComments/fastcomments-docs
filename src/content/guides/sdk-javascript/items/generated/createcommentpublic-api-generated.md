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
const tenantId: string = "tenant_92b4f3";
const urlId: string = "https://www.news-site.com/articles/2026/typescript-updates";
const broadcastId: string = "broadcast_live_20260112";
const commentData: CommentData = {
  body: "Great updates — the stricter inference and tooling improvements will help a lot.",
  author: { name: "Jordan Lee", avatarUrl: "https://avatars.example.com/jordan.jpg" },
  metadata: { client: "web", editor: "rich-text" }
};
const sessionId: string = "sess_7a9c2d4b";
const sso: string = "sso_token_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
const result: CreateCommentPublic200Response = await createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso);
[inline-code-end]
