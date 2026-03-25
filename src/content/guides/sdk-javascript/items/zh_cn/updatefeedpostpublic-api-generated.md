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
const tenantId: string = "tenant_9f4b2";
const postId: string = "post_21a8e";
const updateFeedPostParams: UpdateFeedPostParams = {
  title: "Quarterly product update",
  body: "Major performance improvements and bug fixes deployed today. See release notes and schedule.",
  links: [{ url: "https://status.example.com/release-notes", title: "Release notes" }],
  media: [
    {
      type: "image",
      assets: [{ url: "https://cdn.example.com/updates/q2.png", mimeType: "image/png", width: 1200, height: 628 }]
    }
  ]
};
const broadcastId: string = "broadcast_live_202603";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature";
const result: CreateFeedPostPublic200Response = await updateFeedPostPublic(tenantId, postId, updateFeedPostParams, broadcastId, sso);
[inline-code-end]

---