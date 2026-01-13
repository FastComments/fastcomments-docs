## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Нет |  |

## Ответ

Возвращает: [`Option[CreateEmailTemplate_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_email_template200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример createEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createEmailTemplate(tenantId = "my-tenant-123", createEmailTemplateBody = CreateEmailTemplateBody(name = "Weekly Newsletter", subject = "Weekly updates from OurSite", fromName = "OurSite Team", fromEmail = "newsletter@oursite.com", bodyHtml = "<h1>Highlights</h1><p>Top stories this week...</p>", enabled = true, tags = @["newsletter", "weekly"]))
if response.isSome:
  let template = response.get()
  discard template
[inline-code-end]

---