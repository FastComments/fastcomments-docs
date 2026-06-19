## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createFeedPostParams | CreateFeedPostParams | はい |  |
| broadcastId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostResponse.ts)

## 例

[inline-code-attrs-start title = 'createFeedPostPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f2c3b91";
const createFeedPostParams: CreateFeedPostParams = {
  title: "New mobile app update",
  body: "Version 3.2 adds offline support, faster sync, and improved accessibility.",
  media: [
    {
      type: "image",
      caption: "Offline mode preview",
      asset: {
        url: "https://cdn.company.com/images/app-offline-3-2.png",
        mimeType: "image/png",
        width: 1080,
        height: 2340
      }
    }
  ],
  links: [{ url: "https://blog.company.com/release-notes-3-2", title: "Read full release notes" }],
  tags: ["release", "mobile", "performance"],
  notifySubscribers: true,
  scheduledAt: "2026-06-20T10:00:00Z"
};
const broadcastId: string = "broadcast_9a8b7c";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakePayload.signature";
const response: CreateFeedPostResponse = await createFeedPostPublic(tenantId, createFeedPostParams, broadcastId, sso);
[inline-code-end]

---