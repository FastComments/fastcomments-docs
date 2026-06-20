## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| commentId | string | Sì |  |
| banEmail | bool | No |  |
| banEmailDomain | bool | No |  |
| banIP | bool | No |  |
| deleteAllUsersComments | bool | No |  |
| bannedUntil | string | No |  |
| isShadowBan | bool | No |  |
| updateId | string | No |  |
| banReason | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di postBanUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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