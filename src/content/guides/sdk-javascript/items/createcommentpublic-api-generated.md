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
const tenantId: string = 'tenant_92c8b3';
const urlId: string = 'news-20260109-tech-trends';
const broadcastId: string = 'broadcast_webinar_01-20260109';
const commentData: CommentData = {
  content: "Great piece — I appreciated the section on responsible AI deployment.",
  authorName: "Jordan Lee",
  language: "en",
  metadata: { browser: "Firefox 121", country: "US" }
};
const sessionId: string = 'sess_3f7a9d2e';
const sso: string = 'sso_jwt_eyJhbGciOiJIUzI1NiIk...';
const result: CreateCommentPublic200Response = await createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso);
[inline-code-end]
