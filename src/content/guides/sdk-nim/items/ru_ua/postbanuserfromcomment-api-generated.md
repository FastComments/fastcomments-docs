## Параметры

| Name | Type | Обязательно | Description |
|------|------|------------|-------------|
| commentId | string | Да |  |
| banEmail | bool | Нет |  |
| banEmailDomain | bool | Нет |  |
| banIP | bool | Нет |  |
| deleteAllUsersComments | bool | Нет |  |
| bannedUntil | string | Нет |  |
| isShadowBan | bool | Нет |  |
| updateId | string | Нет |  |
| banReason | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Пример

[inline-code-attrs-start title = 'Пример postBanUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postBanUserFromComment(
  commentId = "cmt-8f3a1b",
  banEmail = false,
  banEmailDomain = false,
  banIP = false,
  deleteAllUsersComments = false,
  bannedUntil = "",
  isShadowBan = false,
  updateId = "",
  banReason = "",
  sso = ""
)
if response.isSome:
  let banResult = response.get()
  discard banResult
else:
  echo "No ban result returned"
[inline-code-end]

---