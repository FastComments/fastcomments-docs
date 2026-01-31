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
const tenantId: string = "acme-pub-01";
const newComment: CreateCommentParams = {
  threadId: "post-2026-01-09",
  content: "Great analysis — I especially liked the section on caching and edge cases.",
  user: { id: "user_824", displayName: "Ava Chen", email: "ava.chen@example.com" },
  clientMetadata: { platform: "web", userAgent: "Mozilla/5.0", locale: "en-US" }
};
const saved: SaveComment200Response = await saveComment(tenantId, newComment, true, true, false, true);
[inline-code-end]
