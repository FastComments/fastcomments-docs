## Параметри

| Назив | Тип | Потребно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Не |  |

## Одговор

Враћа: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Пример

[inline-code-attrs-start title = 'updateEmailTemplate Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateEmailTemplateBody(
  name = "Welcome Email",
  subject = "Welcome to Example News",
  html = "<p>Hi {name}, welcome to Example News.</p>",
  isActive = true,
  tags = @["onboarding", "welcome"]
)

let (response, httpResponse) = client.updateEmailTemplate(
  tenantId = "my-tenant-123",
  id = "welcome-template-2026",
  updateEmailTemplateBody = updateBody
)

if response.isSome:
  let template = response.get()
  discard template
[inline-code-end]

---