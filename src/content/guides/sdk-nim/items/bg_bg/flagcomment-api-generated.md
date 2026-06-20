## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| userId | string | Не |  |
| anonUserId | string | Не |  |

## Отговор

Връща: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Пример

[inline-code-attrs-start title = 'flagComment Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagComment(
  tenantId = "my-tenant-123",
  id = "cmt-98765",
  userId = "user-12345",
  anonUserId = ""
)

if response.isSome:
  let flagResp = response.get()
  echo "Flag response received"
else:
  echo "No flag response returned"
[inline-code-end]

---