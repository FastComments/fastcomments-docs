## Parameters

| Име | Тип | Задължително | Описание |
|------|------|--------------|-----------|
| tenantId | string | Да |  |
| id | string | Не |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Не |  |
| updateComments | bool | Не |  |

## Response

Връща: [`Option[PatchSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_sso_user_api_response.nim)

## Example

[inline-code-attrs-start title = 'Пример за patchSSOUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateData = UpdateAPISSOUserData()
let (responseOpt, httpResponse) = client.patchSSOUser(
  tenantId = "my-tenant-123",
  id = "user-789",
  updateAPISSOUserData = updateData,
  updateComments = true)
if responseOpt.isSome:
  let response = responseOpt.get()
  echo response
echo httpResponse.statusCode
[inline-code-end]