## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| isLive | boolean | No |  |
| doSpamCheck | boolean | No |  |
| skipDupCheck | boolean | No |  |

## Response

Returns: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPost200Response.ts)

## Example

[inline-code-attrs-start title = 'createFeedPost Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-01';
const mediaAsset: FeedPostMediaItemAsset = { url: 'https://cdn.acme.com/images/launch-hero.jpg', mimeType: 'image/jpeg', width: 1600, height: 900 };
const mediaItem: FeedPostMediaItem = { id: 'm-001', type: 'image', assets: [mediaAsset], altText: 'Acme product launch on stage' };
const link: FeedPostLink = { url: 'https://acme.com/blog/product-launch', title: 'Read the full launch post' };
const createFeedPostParams: CreateFeedPostParams = {
  title: 'Introducing Acme One',
  body: 'Today we launched Acme One — a unified platform to streamline workflows for teams of all sizes.',
  media: [mediaItem],
  links: [link],
  visibility: 'public'
};
const broadcastId: string = 'broadcast_20260109_01';
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const skipDupCheck: boolean = false;
const response: CreateFeedPost200Response = await createFeedPost(tenantId, createFeedPostParams, broadcastId, isLive, doSpamCheck, skipDupCheck);
[inline-code-end]
