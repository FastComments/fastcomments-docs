## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| postId | string | Так |  |
| broadcastId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublicResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-7f3b';
const postId: string = 'post_589132';
const broadcastId: string = 'broadcast_2026-06-19_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NDMyMSIsIm5hbWUiOiJKb2huIERvZSJ9.DX3h7k9vYz0Qx2p5u1L8b6c9R4s';
const result: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
[inline-code-end]

---