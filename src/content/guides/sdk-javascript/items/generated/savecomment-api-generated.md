## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | CreateCommentParams | Yes |  |
| isLive | boolean | No |  |
| doSpamCheck | boolean | No |  |
| sendEmails | boolean | No |  |
| populateNotifications | boolean | No |  |

## Response

Returns: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveComment200Response.ts)

## Example

[inline-code-attrs-start title = 'saveComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_92b3c";
const mention: CommentUserMentionInfo = { userId: "u842", displayName: "Alex Chen" };
const hashtag: CommentUserHashTagInfo = { tag: "performance" };
const createCommentParams: CreateCommentParams = {
  threadId: "article-frontend-performance-2026",
  content: "Performance improvements look great — shipped this in prod and saw a 12% render time reduction.",
  authorName: "Jordan Harper",
  authorEmail: "jordan.harper@example.com",
  mentions: [mention],
  hashtags: [hashtag]
};
const result: SaveComment200Response = await saveComment(tenantId, createCommentParams, true, true, false, true);
[inline-code-end]
