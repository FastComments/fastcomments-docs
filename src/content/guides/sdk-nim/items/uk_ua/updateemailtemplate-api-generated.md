## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Ні |  |

## Відповідь

Повертає: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад updateEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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