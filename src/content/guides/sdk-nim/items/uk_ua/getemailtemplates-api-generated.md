## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| skip | float64 | Ні |  |

## Відповідь

Повертає: [`Option[GetEmailTemplates_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_templates200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getEmailTemplates'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplates(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let templates = response.get()
  echo templates
else:
  echo "No templates returned"
[inline-code-end]

---