## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`Option[GetBannedUsersFromCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_from_comment_response.nim)

## Örnek

[inline-code-attrs-start title = 'getBanUsersFromComment Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getBanUsersFromComment(commentId = "comment-98765", sso = "")
if response.isSome:
  let bannedResp = response.get()
  discard bannedResp
else:
  echo "No banned users found or request failed"
[inline-code-end]

---