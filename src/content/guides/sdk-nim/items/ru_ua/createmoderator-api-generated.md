## Параметри

| Назва | Тип | Обовʼязковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| createModeratorBody | CreateModeratorBody | Ні |  |

## Відповідь

Повертає: [`Option[CreateModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator_response.nim)

## Приклад

[inline-code-attrs-start title = 'createModerator Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorRes, httpResp) = client.createModerator(tenantId = "my-tenant-123", createModeratorBody = CreateModeratorBody())
if moderatorRes.isSome:
  let moderator = moderatorRes.get()
[inline-code-end]