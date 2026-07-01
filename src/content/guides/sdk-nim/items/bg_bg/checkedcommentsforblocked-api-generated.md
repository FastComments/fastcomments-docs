## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| commentIds | string | Не |  |
| sso | string = "" | Не |  |

## Отговор

Връща: [`Option[CheckBlockedCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_check_blocked_comments_response.nim)

## Пример

[inline-code-attrs-start title = 'checkedCommentsForBlocked Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.checkedCommentsForBlocked(
  tenantId = "my-tenant-123",
  commentIds = "cmt-1,cmt-2",
  sso = ""
)

if maybeResponse.isSome:
  let response = maybeResponse.get()
  discard response
[inline-code-end]