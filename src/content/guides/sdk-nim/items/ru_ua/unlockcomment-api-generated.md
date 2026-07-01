## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| broadcastId | string | Нет |  |
| sso | string = "" | Нет |  |

## Ответ

Возвращает: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример unLockComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeEmpty, httpResp) = client.unLockComment(tenantId = "my-tenant-123", commentId = "comment-456", broadcastId = "", sso = "")
if maybeEmpty.isSome:
  let emptyResp = maybeEmpty.get()
  echo "Comment unlocked"
else:
  echo "Failed to unlock comment"
[inline-code-end]