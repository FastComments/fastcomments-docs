## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postId | string | 是 |  |
| updateFeedPostParams | UpdateFeedPostParams | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'updateFeedPostPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const postId: string = 'post_20260615_001';
const updateFeedPostParams: UpdateFeedPostParams = {
  title: 'Weekly Update: Product Launch',
  content: 'We shipped the 2.0 release today — highlights and links below.',
  media: [{ url: 'https://cdn.acme.com/releases/launch.jpg', type: 'image' }],
  tags: ['release', 'product'],
  isPublic: true
};
const broadcastId: string = 'broadcast_live_42';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const result: CreateFeedPostPublic200Response = await updateFeedPostPublic(tenantId, postId, updateFeedPostParams, broadcastId, sso);
[inline-code-end]

---