## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Не |  |

## Одговор

Враћа: [`Option[CreateEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_email_template_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример createEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createEmailTemplate(tenantId = "my-tenant-123",
  createEmailTemplateBody = CreateEmailTemplateBody(
    name = "Weekly Newsletter",
    subject = "This Week on NewsSite",
    html = "<h1>Latest updates</h1><p>Read our latest article.</p>",
    fromAddress = "no-reply@newssite.com",
    isDefault = false,
    tags = @["news", "weekly"]
  )
)

if response.isSome:
  let template = response.get()
  echo "Created email template id: ", template.id
[inline-code-end]