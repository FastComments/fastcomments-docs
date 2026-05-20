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
const tenantId: string = 'tenant_4821';
const id: string = 'post_9b34c';
const mediaAsset: FeedPostMediaItemAsset = { url: 'https://cdn.company.com/assets/launch.jpg', mimeType: 'image/jpeg' };
const mediaItem: FeedPostMediaItem = { url: 'https://cdn.company.com/assets/launch.jpg', type: 'image', assets: [mediaAsset], caption: 'Product launch hero image' };
const link: FeedPostLink = { url: 'https://company.com/blog/product-launch', title: 'Read full announcement' };
const feedPost: FeedPost = { title: 'Product Launch', content: 'We are excited to announce the new version of our product with enhanced performance and security.', media: [mediaItem], links: [link] };
const result: FlagCommentPublic200Response = await updateFeedPost(tenantId, id, feedPost);
[inline-code-end]
