---
ページの現在オンラインの閲覧者: 今まさにそのページに対してWebSocketセッションがサブスクライブされているユーザー。  
anonCount + totalCount を返します（ルーム全体の購読者数。個々に列挙しない匿名閲覧者を含む）。

## パラメーター

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| afterName | string | いいえ |  |
| afterUserId | string | いいえ |  |

## レスポンス

返却値: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## 例

[inline-code-attrs-start title = 'getOnlineUsers の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/politics/top-story", afterName = "", afterUserId = "")
if response.isSome:
  let page = response.get()
  echo "Received online users page:"
  echo page
else:
  echo "No online users returned. HTTP status: ", httpResponse.statusCode
[inline-code-end]

---