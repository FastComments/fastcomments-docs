---
現在オンラインではない、そのページの過去のコメント投稿者。displayNameでソートされています。
/users/online を使い果たした後に、"Members" セクションをレンダリングするために使用します。
commenterName に対するカーソルページネーション: サーバーは部分的な {tenantId, urlId, commenterName} を走査します。
afterName から先へ $gt を使ってインデックスし、$skip のコストは発生しません。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| afterName | string | いいえ |  |
| afterUserId | string | いいえ |  |

## レスポンス

戻り値: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---