## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPISSOUserData | CreateAPISSOUserData | No |  |

## Отговор

Връща: [`Option[AddSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_sso_user_api_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за addSSOUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.addSSOUser(tenantId = "my-tenant-123", createAPISSOUserData = CreateAPISSOUserData())
if optResp.isSome:
  let userResp = optResp.get()
[inline-code-end]