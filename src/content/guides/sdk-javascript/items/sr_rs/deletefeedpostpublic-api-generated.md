## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| postId | string | Да |  |
| broadcastId | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublicResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteFeedPostPublic Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-7f3b';
const postId: string = 'post_589132';
const broadcastId: string = 'broadcast_2026-06-19_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NDMyMSIsIm5hbWUiOiJKb2huIERvZSJ9.DX3h7k9vYz0Qx2p5u1L8b6c9R4s';
const result: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
[inline-code-end]

---