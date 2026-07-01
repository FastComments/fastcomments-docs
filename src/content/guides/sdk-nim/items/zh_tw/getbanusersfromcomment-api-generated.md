## 參數

| 名稱 | 類型 | 必須 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| sso | string = "" | 否 |  |

## 回應

返回: [`Option[GetBannedUsersFromCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_from_comment_response.nim)

## 範例

[inline-code-attrs-start title = 'getBanUsersFromComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getBanUsersFromComment(tenantId = "my-tenant-001", commentId = "cmt-123456", sso = "")
if response.isSome:
  let banInfo = response.get()
  echo banInfo
[inline-code-end]

---