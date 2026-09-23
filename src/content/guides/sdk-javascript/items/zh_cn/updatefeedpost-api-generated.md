## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| feedPost | FeedPost | 是 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'updateFeedPost 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
  const tenantId: string = "tenant_12345";
  const postId: string = "post_98765";

  const mediaAsset: FeedPostMediaItemAsset = {
    url: "https://cdn.example.com/image.jpg",
    width: 800,
    height: 600,
    mimeType: "image/jpeg"
  };

  const mediaItem: FeedPostMediaItem = {
    type: "image",
    asset: mediaAsset,
    caption: "Sunrise over the hills"
  };

  const link: FeedPostLink = {
    url: "https://example.com/blog",
    title: "Latest Blog Post",
    description: "Insights on recent developments"
  };

  const feedPost: FeedPost = {
    title: "Daily Highlights",
    content: "Check out today’s top stories.",
    mediaItems: [mediaItem], // optional
    links: [link] // optional
  };

  const response: APIEmptyResponse = await updateFeedPost(tenantId, postId, feedPost);
  console.log(response);
}

runUpdate();
[inline-code-end]