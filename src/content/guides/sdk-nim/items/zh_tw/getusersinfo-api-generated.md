---
批次取得租戶的使用者資訊。根據 userIds，回傳來自 User / SSOUser 的顯示資訊。  
此功能被評論小工具使用，以在使用者透過在場事件剛出現時豐富其資訊。  
無頁面上下文：隱私權會一致性地被強制執行（私人檔案會被遮蔽）。

## Parameters

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| ids | string | 否 |  |

## 回應

返回：[`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## 範例

[inline-code-attrs-start title = 'getUsersInfo 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]

---