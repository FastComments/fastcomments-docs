## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |

## Ответ

Возвращает: [`Option[GetEmailTemplateDefinitionsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_definitions_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getEmailTemplateDefinitions'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.getEmailTemplateDefinitions(tenantId = "my-tenant-123")
if responseOpt.isSome:
  let definitions = responseOpt.get()
  echo definitions
[inline-code-end]

---