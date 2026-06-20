## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| commentId | string | Oui |  |
| banEmail | bool | Non |  |
| banEmailDomain | bool | Non |  |
| banIP | bool | Non |  |
| deleteAllUsersComments | bool | Non |  |
| bannedUntil | string | Non |  |
| isShadowBan | bool | Non |  |
| updateId | string | Non |  |
| banReason | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de postBanUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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