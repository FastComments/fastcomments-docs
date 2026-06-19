---
ページの現在オンラインの閲覧者: 現在そのページに対して WebSocket セッションが購読されているユーザー。anonCount + totalCount を返します（ルーム全体の購読者数。個別に列挙しない匿名閲覧者を含む）。

## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| afterName | string | いいえ |  |
| afterUserId | string | いいえ |  |

## レスポンス

返却: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## 例

[inline-code-attrs-start title = 'getOnlineUsers の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---