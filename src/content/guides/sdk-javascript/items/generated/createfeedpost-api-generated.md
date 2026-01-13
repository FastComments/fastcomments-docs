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
const tenantId: string = "tenant_8f3b2c";
const createFeedPostParams: CreateFeedPostParams = {
  title: "Weekly product update",
  body: "We shipped performance improvements and a login fix. Please share feedback.",
  authorId: "user_72a1",
  externalId: "post-2026-01-12",
  media: [
    {
      type: "image",
      assets: [
        {
          url: "https://cdn.example.com/images/update-jan.jpg",
          width: 1200,
          height: 630,
          mimeType: "image/jpeg"
        } as FeedPostMediaItemAsset
      ]
    } as FeedPostMediaItem
  ],
  links: [
    { url: "https://example.com/releases/january", title: "Release notes" } as FeedPostLink
};
const broadcastId: string = "bcast_20260112";
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const skipDupCheck: boolean = false;
const response: CreateFeedPost200Response = await createFeedPost(tenantId, createFeedPostParams, broadcastId, isLive, doSpamCheck, skipDupCheck);
[inline-code-end]
