## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад updateEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateEmailTemplateBody(
  subject: "Welcome to FastComments",
  body: "Hello \{{user_name}}, thanks for joining!",
  enabled: true,
)

let (maybeResp, httpResp) = client.updateEmailTemplate(
  tenantId = "my-tenant-123",
  id = "welcome-email",
  updateEmailTemplateBody = updateBody,
)

if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]