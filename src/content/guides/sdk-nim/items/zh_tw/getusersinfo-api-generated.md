租戶的批次使用者資訊。給定 userIds，回傳來自 User / SSOUser 的顯示資訊。用於留言元件以豐富剛透過 presence 事件出現的使用者。無頁面上下文：隱私一律強制執行（私人檔案會被遮罩）。

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| ids | string | 否 |  |

## 回應

回傳: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## 範例

[inline-code-attrs-start title = 'getUsersInfo 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---