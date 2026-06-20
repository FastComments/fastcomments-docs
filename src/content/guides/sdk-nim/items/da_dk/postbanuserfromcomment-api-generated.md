## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| banEmail | bool | Nej |  |
| banEmailDomain | bool | Nej |  |
| banIP | bool | Nej |  |
| deleteAllUsersComments | bool | Nej |  |
| bannedUntil | string | Nej |  |
| isShadowBan | bool | Nej |  |
| updateId | string | Nej |  |
| banReason | string | Nej |  |
| sso | string | Nej |  |

## Respons

Returnerer: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på postBanUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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