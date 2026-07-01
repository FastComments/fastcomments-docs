---
## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| options | DeleteSSOUserOptions | 否 |  |

## 回應

返回: [`Option[DeleteSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_sso_user_api_response.nim)

## 範例

[inline-code-attrs-start title = 'deleteSSOUser 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRespOpt, httpResp) = client.deleteSSOUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  options = DeleteSSOUserOptions()
)

if apiRespOpt.isSome:
  let apiResp = apiRespOpt.get()
[inline-code-end]

---