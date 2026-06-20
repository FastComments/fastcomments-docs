## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| commentId | string | Так |  |
| banEmail | bool | Ні |  |
| banEmailDomain | bool | Ні |  |
| banIP | bool | Ні |  |
| deleteAllUsersComments | bool | Ні |  |
| bannedUntil | string | Ні |  |
| isShadowBan | bool | Ні |  |
| updateId | string | Ні |  |
| banReason | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад postBanUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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