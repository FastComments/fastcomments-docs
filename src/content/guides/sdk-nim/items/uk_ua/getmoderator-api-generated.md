## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| id | string | Ні |  |

## Відповідь

Повертає: [`Option[GetModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorOpt, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "moderator-456")
if moderatorOpt.isSome:
  let moderator = moderatorOpt.get()
  discard moderator
[inline-code-end]