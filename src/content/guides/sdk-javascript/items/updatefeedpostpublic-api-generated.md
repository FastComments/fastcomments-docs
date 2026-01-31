## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| updateFeedPostParams | UpdateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateFeedPostPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-8c3f4b2a";
const postId: string = "post-0d9a7b2e";
const mediaAsset: FeedPostMediaItemAsset = {
  url: "https://cdn.example.com/images/update-2-1.png",
  mimeType: "image/png",
  width: 1200,
  height: 628
};
const mediaItem: FeedPostMediaItem = {
  type: "image",
  assets: [mediaAsset],
  caption: "Version 2.1 UI refresh"
};
const updateFeedPostParams: UpdateFeedPostParams = {
  title: "Weekly product update — v2.1",
  content: "Released v2.1 with performance improvements, bug fixes, and accessibility updates.",
  media: [mediaItem],
  links: [{ url: "https://example.com/release-notes/2.1", title: "Release notes" }],
  isPublished: true
};
const broadcastId: string = "broadcast-20260109";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload";
const result: CreateFeedPostPublic200Response = await updateFeedPostPublic(tenantId, postId, updateFeedPostParams, broadcastId, sso);
[inline-code-end]
