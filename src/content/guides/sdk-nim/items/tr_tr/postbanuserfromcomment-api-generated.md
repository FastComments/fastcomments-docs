## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| banEmail | bool | Hayır |  |
| banEmailDomain | bool | Hayır |  |
| banIP | bool | Hayır |  |
| deleteAllUsersComments | bool | Hayır |  |
| bannedUntil | string | Hayır |  |
| isShadowBan | bool | Hayır |  |
| updateId | string | Hayır |  |
| banReason | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Örnek

[inline-code-attrs-start title = 'postBanUserFromComment Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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