## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| banEmail | bool | Nee |  |
| banEmailDomain | bool | Nee |  |
| banIP | bool | Nee |  |
| deleteAllUsersComments | bool | Nee |  |
| bannedUntil | string | Nee |  |
| isShadowBan | bool | Nee |  |
| updateId | string | Nee |  |
| banReason | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Voorbeeld

[inline-code-attrs-start title = 'postBanUserFromComment Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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