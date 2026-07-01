## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|---------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| sso | string = "" | Nee |  |

## Reactie

Retourneert: [`Option[GetBannedUsersFromCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_from_comment_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getBanUsersFromComment Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getBanUsersFromComment(tenantId = "my-tenant-001", commentId = "cmt-123456", sso = "")
if response.isSome:
  let banInfo = response.get()
  echo banInfo
[inline-code-end]