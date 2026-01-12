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
const tenantId: string = 'tenant-42-west';
const id: string = 'post-2026-01-12-007';
const mediaItem: FeedPostMediaItem = {
  type: 'image',
  caption: 'Team at launch event',
  assets: [{ url: 'https://cdn.fastcomments.example/photos/launch-1234.jpg', width: 1920, height: 1080 }]
};
const feedPost: FeedPost = {
  title: 'Product Launch Recap',
  body: 'We rolled out the new moderation features and performance improvements.',
  published: true,
  // optional parameters demonstrated:
  media: [mediaItem],
  links: [{ title: 'Full changelog', url: 'https://example.com/changelog/q1-2026' }]
};
const response: FlagCommentPublic200Response = await updateFeedPost(tenantId, id, feedPost);
[inline-code-end]
