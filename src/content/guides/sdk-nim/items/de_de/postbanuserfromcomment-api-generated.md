## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| banEmail | bool | Nein |  |
| banEmailDomain | bool | Nein |  |
| banIP | bool | Nein |  |
| deleteAllUsersComments | bool | Nein |  |
| bannedUntil | string | Nein |  |
| isShadowBan | bool | Nein |  |
| updateId | string | Nein |  |
| banReason | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Beispiel

[inline-code-attrs-start title = 'postBanUserFromComment Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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