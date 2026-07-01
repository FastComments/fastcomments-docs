## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Не |  |
| updateComments | bool | Не |  |

## Отговор

Връща: [`Option[PutSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_put_sso_user_api_response.nim)

## Пример

[inline-code-attrs-start title = 'putSSOUser Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRespOpt, httpResp) = client.putSSOUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateAPISSOUserData = default(UpdateAPISSOUserData),
  updateComments = false)

if apiRespOpt.isSome:
  let apiResp = apiRespOpt.get()
  echo apiResp
[inline-code-end]