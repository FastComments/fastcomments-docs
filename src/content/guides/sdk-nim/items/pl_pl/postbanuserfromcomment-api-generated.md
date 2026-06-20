## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| commentId | string | Tak |  |
| banEmail | bool | Nie |  |
| banEmailDomain | bool | Nie |  |
| banIP | bool | Nie |  |
| deleteAllUsersComments | bool | Nie |  |
| bannedUntil | string | Nie |  |
| isShadowBan | bool | Nie |  |
| updateId | string | Nie |  |
| banReason | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład postBanUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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