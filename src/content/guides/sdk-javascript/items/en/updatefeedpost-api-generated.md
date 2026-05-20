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
const tenantId: string = 'acme-tenant-01';
const id: string = 'feedpost-2026-0001';
const mediaAsset: FeedPostMediaItemAsset = { assetId: 'asset-9876', url: 'https://cdn.acme.com/assets/asset-9876.jpg' } as FeedPostMediaItemAsset;
const mediaItem: FeedPostMediaItem = { id: 'media-1', type: 'image', asset: mediaAsset } as FeedPostMediaItem;
const feedPost: FeedPost = { title: 'Product Launch', body: 'We launched the new product today.', media: [mediaItem], isPinned: true } as FeedPost;
const result: FlagCommentPublic200Response = await updateFeedPost(tenantId, id, feedPost);
[inline-code-end]
