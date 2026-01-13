## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| errorId | string | Не |  |

## Одговор

Враћа: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Примјер

[inline-code-attrs-start title = 'Примјер deleteEmailTemplateRenderError'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteEmailTemplateRenderError(
  tenantId = "my-tenant-123",
  id = "welcome-email-template",
  errorId = "render-error-2026"
)
if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]

---