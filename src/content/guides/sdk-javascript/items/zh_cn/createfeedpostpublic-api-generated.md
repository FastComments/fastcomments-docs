## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## 响应

返回：[`CreateFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostPublicResponse.ts)

## 示例

[inline-code-attrs-start title = 'createFeedPostPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function submitPost() {
  const tenantId: string = "tenant_12345";

  const mediaItem: FeedPostMediaItem = {
    asset: {
      url: "https://cdn.example.com/image.jpg",
      mimeType: "image/jpeg",
      size: 104857,
    },
    type: "image",
  };

  const link: FeedPostLink = {
    url: "https://example.com/blog/new-feature",
    title: "New Feature Release",
    description: "Details about our latest product update.",
  };

  const createFeedPostParams: CreateFeedPostParams = {
    content: "Check out our new feature!",
    media: [mediaItem],
    link: link,
  };

  const broadcastId: string = "broadcast_9876";
  const sso: string = "sso_token_abc123";

  const response: CreateFeedPostPublicResponse = await createFeedPostPublic(
    tenantId,
    createFeedPostParams,
    broadcastId,
    sso
  );
}
[inline-code-end]