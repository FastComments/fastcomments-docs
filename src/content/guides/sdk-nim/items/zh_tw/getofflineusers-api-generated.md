---
頁面上曾留言但目前不在線上的使用者。依 displayName 排序。
在用盡 /users/online 後使用，以呈現「成員」區段。
以 commenterName 做游標分頁：伺服器會於部分索引 {tenantId, urlId, commenterName} 從 afterName 向前透過 $gt 遍歷，沒有 $skip 的成本。

## 參數

| 名稱 | 類型 | 是否必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| afterName | string | 否 |  |
| afterUserId | string | 否 |  |

## 回應

回傳: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## 範例

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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