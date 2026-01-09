## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| deleteComments | bool | 否 |  |
| commentDeleteMode | string | 否 |  |

## 回應

回傳: [`Option[DeleteSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_sso_user_api_response.nim)

## 範例

[inline-code-attrs-start title = 'deleteSSOUser 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteSSOUser(tenantId = "my-tenant-123", id = "sso-user-9876", deleteComments = true, commentDeleteMode = "hard")
if response.isSome:
  let deleted = response.get()
  discard deleted
else:
  discard httpResponse
[inline-code-end]

---