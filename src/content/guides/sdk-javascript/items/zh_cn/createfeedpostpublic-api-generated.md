## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createFeedPostParams | CreateFeedPostParams | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostResponse.ts)

## 示例

[inline-code-attrs-start title = 'createFeedPostPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const mediaAsset: FeedPostMediaItemAsset = {
  url: "https://cdn.example.com/image.jpg",
  width: 800,
  height: 600,
  mimeType: "image/jpeg"
};

const mediaItem: FeedPostMediaItem = {
  type: "image",
  asset: mediaAsset,
  caption: "Sunset over the hills"
};

const link: FeedPostLink = {
  url: "https://example.com/blog/post",
  title: "Exciting New Features",
  description: "Read about our latest updates."
};

const createFeedPostParams: CreateFeedPostParams = {
  content: "Check out our latest blog post!",
  media: [mediaItem],
  link: link,
  visibility: "public"
};

const broadcastId: string = "broadcast_9876";
const sso: string = "sso_token_abc123";

const response: CreateFeedPostResponse = await createFeedPostPublic(
  tenantId,
  createFeedPostParams,
  broadcastId,
  sso
);
[inline-code-end]