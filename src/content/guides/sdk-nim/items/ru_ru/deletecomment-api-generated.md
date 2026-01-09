## Параметры

| Имя | Type | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |
| contextUserId | string | Нет |  |
| isLive | bool | Нет |  |

## Ответ

Возвращает: [`Option[DeleteComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример deleteComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteComment(tenantId = "my-tenant-123", id = "cmt-456abc", contextUserId = "user-789", isLive = true)
if response.isSome:
  let deleted = response.get()
  discard deleted
  echo "Delete succeeded"
else:
  echo "No delete response"
[inline-code-end]

---