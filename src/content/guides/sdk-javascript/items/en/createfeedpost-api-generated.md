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
const tenantId: string = 'tenant-7421';
const imageAsset: FeedPostMediaItemAsset = { url: 'https://cdn.example.com/images/launch.jpg', mimeType: 'image/jpeg', width: 1200, height: 630 };
const mediaItem: FeedPostMediaItem = { type: 'image', assets: [imageAsset], caption: 'Highlights from today’s release' };
const link: FeedPostLink = { url: 'https://www.example.com/releases/v2', title: 'Release notes' };
const createFeedPostParams: CreateFeedPostParams = { title: 'Version 2.0 Released', body: 'We shipped major performance improvements and new features.', media: [mediaItem], link };
const broadcastId: string = 'broadcast-602';
const isLive: boolean = false;
const doSpamCheck: boolean = true;
const result: CreateFeedPost200Response = await createFeedPost(tenantId, createFeedPostParams, broadcastId, isLive, doSpamCheck);
[inline-code-end]
