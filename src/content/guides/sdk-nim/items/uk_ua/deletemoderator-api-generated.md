## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|---------------|------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| sendEmail | string = "" | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.deleteModerator(
  tenantId = "my-tenant-123",
  id = "mod-789",
  sendEmail = "admin@mydomain.com",
)

if apiResp.isSome:
  let empty = apiResp.get()
  echo "Moderator removed"
[inline-code-end]