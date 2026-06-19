---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postId | string | 是 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublicResponse.ts)

## 範例

[inline-code-attrs-start title = 'deleteFeedPostPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-7f3b';
const postId: string = 'post_589132';
const broadcastId: string = 'broadcast_2026-06-19_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NDMyMSIsIm5hbWUiOiJKb2huIERvZSJ9.DX3h7k9vYz0Qx2p5u1L8b6c9R4s';
const result: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
[inline-code-end]

---