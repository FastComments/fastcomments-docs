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
const tenantId: string = "tenant_42_acme_corp";
const assets: FeedPostMediaItemAsset[] = [{ url: "https://cdn.acme.com/images/launch.jpg", mimeType: "image/jpeg" }];
const mediaItems: FeedPostMediaItem[] = [{ type: "image", caption: "Launch hero image", assets }];
const links: FeedPostLink[] = [{ url: "https://acme.com/products/new", title: "Product details" }];
const createFeedPostParams: CreateFeedPostParams = {
  authorId: "user_987",
  content: "We're live with the new product release. Join the demo stream.",
  media: mediaItems,
  links
};
const broadcastId: string = "broadcast_20260520_01";
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const skipDupCheck: boolean = false;
const result: CreateFeedPost200Response = await createFeedPost(tenantId, createFeedPostParams, broadcastId, isLive, doSpamCheck, skipDupCheck);
[inline-code-end]
