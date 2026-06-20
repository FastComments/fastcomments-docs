## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[GetBannedUsersFromCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_from_comment_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getBanUsersFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getBanUsersFromComment(commentId = "comment-98765", sso = "")
if response.isSome:
  let bannedResp = response.get()
  discard bannedResp
else:
  echo "No banned users found or request failed"
[inline-code-end]

---