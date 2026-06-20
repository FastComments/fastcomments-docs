## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| commentId | string | Da |  |
| banEmail | bool | Ne |  |
| banEmailDomain | bool | Ne |  |
| banIP | bool | Ne |  |
| deleteAllUsersComments | bool | Ne |  |
| bannedUntil | string | Ne |  |
| isShadowBan | bool | Ne |  |
| updateId | string | Ne |  |
| banReason | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer postBanUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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