## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createAPISSOUserData | CreateAPISSOUserData | Не |  |

## Одговор

Враћа: [`Option[AddSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_sso_user_api_response.nim)

## Пример

[inline-code-attrs-start title = 'addSSOUser пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.addSSOUser(tenantId = "my-tenant-123", createAPISSOUserData = CreateAPISSOUserData())
if optResp.isSome:
  let userResp = optResp.get()
[inline-code-end]