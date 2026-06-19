ページ上の過去のコメント投稿者（現在オンラインではないユーザー）。displayNameでソートされています。
/users/online を使い切った後に「Members」セクションを表示するためにこれを使用します。
commenterName によるカーソルページネーション: サーバーは部分的な {tenantId, urlId, commenterName}
インデックスを afterName から先へ $gt で進め、$skip のコストは発生しません。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| afterName | string | いいえ |  |
| afterUserId | string | いいえ |  |

## レスポンス

戻り値: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---