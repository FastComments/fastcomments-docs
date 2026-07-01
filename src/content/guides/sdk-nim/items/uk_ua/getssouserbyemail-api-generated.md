---
## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| email | string | Ні |  |

## Відповідь

Повертає: [`Option[GetSSOUserByEmailAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_user_by_email_api_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getSSOUserByEmail'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getSSOUserByEmail(tenantId = "my-tenant-123", email = "john.doe@example.com")
if optResp.isSome:
  let user = optResp.get()
  discard user
[inline-code-end]

---