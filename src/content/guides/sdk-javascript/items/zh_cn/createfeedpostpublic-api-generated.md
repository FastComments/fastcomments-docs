---
## 参数

| 名称 | 类型 | 是否必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createFeedPostParams | CreateFeedPostParams | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'createFeedPostPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_987654321";
const asset: FeedPostMediaItemAsset = { url: "https://cdn.fastcomments.com/uploads/team-photo.jpg", mimeType: "image/jpeg", sizeBytes: 324512 };
const mediaItem: FeedPostMediaItem = { type: "image", assets: [asset], caption: "Team launch day" };
const link: FeedPostLink = { url: "https://www.example.com/blog/product-update-march-2026", title: "Product update — March 2026" };
const createFeedPostParams: CreateFeedPostParams = {
  title: "Product update — March 2026",
  content: "<p>We shipped performance improvements and two new integrations.</p>",
  media: [mediaItem],
  link,
  visibility: "public",
  tags: ["product","release","march-2026"],
  customConfig: { allowComments: true, requireTOS: false }
};
const broadcastId: string = "broadcast_2026_03_25_live";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiIxMjM0NSIsImlhdCI6MTY5MDI0MDB9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const result: CreateFeedPostPublic200Response = await createFeedPostPublic(tenantId, createFeedPostParams, broadcastId, sso);
[inline-code-end]

---