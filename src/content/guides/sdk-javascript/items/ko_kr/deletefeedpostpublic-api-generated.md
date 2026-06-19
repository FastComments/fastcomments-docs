## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postId | string | 예 |  |
| broadcastId | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublicResponse.ts)

## 예제

[inline-code-attrs-start title = 'deleteFeedPostPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-7f3b';
const postId: string = 'post_589132';
const broadcastId: string = 'broadcast_2026-06-19_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NDMyMSIsIm5hbWUiOiJKb2huIERvZSJ9.DX3h7k9vYz0Qx2p5u1L8b6c9R4s';
const result: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
[inline-code-end]

---