## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| badgeId | string | Нет |  |
| userId | string | Нет |  |
| commentId | string | Да |  |
| broadcastId | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример использования putRemoveBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putRemoveBadge(badgeId = "verified-journalist",
  userId = "user-7890",
  commentId = "comment-98765",
  broadcastId = "",
  sso = "")

if response.isSome:
  let removeResp = response.get()
  discard removeResp
else:
  discard httpResponse
[inline-code-end]

---