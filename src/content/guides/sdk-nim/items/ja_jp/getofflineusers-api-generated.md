現在オンラインではない、ページ上の過去のコメント投稿者。displayNameでソートされています。
/users/online を使い切った後に、"Members" セクションをレンダリングするために使用します。
commenterName によるカーソルページネーション: サーバーは部分インデックス {tenantId, urlId, commenterName} を afterName 以降へ $gt で順に走査し、$skip のコストは発生しません。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| afterName | string | いいえ |  |
| afterUserId | string | いいえ |  |

## レスポンス

返却: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-how-to-code",
  afterName = "",
  afterUserId = ""
)

if response.isSome:
  let offlinePage = response.get()
  echo "Received offline users page"
  discard httpResponse.statusCode
[inline-code-end]

---