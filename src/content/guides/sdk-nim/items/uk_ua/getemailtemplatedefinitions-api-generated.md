---
## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Відповідь

Повертає: [`Option[GetEmailTemplateDefinitionsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_definitions_response.nim)

## Приклад

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.getEmailTemplateDefinitions(tenantId = "my-tenant-123")
if responseOpt.isSome:
  let definitions = responseOpt.get()
  echo definitions
[inline-code-end]

---