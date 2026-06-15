## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| feedPost | FeedPost | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateFeedPost Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_72f3b4c9";
const id: string = "post_ba4f6e18-2d3c-4b7a-91f2-8c0e3a6b5d4f";

const feedPost: FeedPost = {
  title: "June feature rollout",
  body: "Announcing performance improvements and moderation updates available to all sites.",
  authorName: "Platform Team",
  mediaItems: [
    {
      type: "image",
      caption: "Release banner",
      asset: { url: "https://cdn.fastcomments.com/assets/june-banner.jpg", mimeType: "image/jpeg", width: 1200, height: 600 }
    }
  ],
  links: [{ title: "Release notes", url: "https://docs.fastcomments.com/releases/june-2026" }]
} as FeedPost;

const result: FlagCommentPublic200Response = await updateFeedPost(tenantId, id, feedPost);
[inline-code-end]
