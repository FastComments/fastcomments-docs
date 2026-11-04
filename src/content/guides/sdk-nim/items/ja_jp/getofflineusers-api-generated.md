現在オンラインではない、そのページの過去のコメント投稿者。displayName でソートされます。
/users/online を使い切った後に、この API を使用して「メンバー」セクションをレンダリングしてください。
commenterName によるカーソルページネーション: サーバーは部分インデックス {tenantId, urlId, commenterName} を afterName から $gt を使って前方に走査し、$skip のコストはかかりません。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| options | GetOfflineUsersOptions | いいえ |  |

## レスポンス

戻り値: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]