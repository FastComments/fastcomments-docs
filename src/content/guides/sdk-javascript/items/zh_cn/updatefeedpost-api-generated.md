## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| feedPost | FeedPost | 是 |  |

## 响应

返回：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'updateFeedPost 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-global-tenant-42';
const id: string = 'f47ac10b-58cc-4372-a567-0e02b2c3d479';

const asset: FeedPostMediaItemAsset = {
  url: 'https://cdn.acme.com/images/product-launch.jpg',
  mimeType: 'image/jpeg',
  width: 1200,
  height: 630
};

const mediaItem: FeedPostMediaItem = {
  id: 'media-001',
  type: 'image',
  asset
};

const link: FeedPostLink = {
  url: 'https://acme.com/blog/product-launch',
  title: 'Product Launch Details'
};

const feedPost: FeedPost = {
  title: 'Introducing the Q3 Product Suite',
  body: 'We are excited to unveil our new lineup for Q3, focusing on performance and security improvements.',
  media: [mediaItem],     // 包含可选数组
  links: [link],          // 包含可选链接
  isPublished: true       // 这里使用了可选的发布标志
};

const result: FlagCommentPublic200Response = await updateFeedPost(tenantId, id, feedPost);
[inline-code-end]

---