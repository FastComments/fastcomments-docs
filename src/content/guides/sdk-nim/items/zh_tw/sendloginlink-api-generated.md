## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| redirectURL | string | 否 |  |

## 回應

回傳: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 範例

[inline-code-attrs-start title = 'sendLoginLink 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.sendLoginLink(tenantId = "fastcomments-tenant-42", id = "user-9876", redirectURL = "https://news.example.com/articles/2026/fastcomments-login")
if response.isSome:
  let loginResp = response.get()
  echo "Login link sent successfully"
else:
  echo "Failed to send login link"
[inline-code-end]