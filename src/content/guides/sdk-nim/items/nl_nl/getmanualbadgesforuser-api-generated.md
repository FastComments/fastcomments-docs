## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| badgesUserId | string | Nee |  |
| commentId | string | Ja |  |
| sso | string | Nee |  |

## Respons

Geeft terug: [`Option[GetUserManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_manual_badges_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getManualBadgesForUser Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getManualBadgesForUser(
  badgesUserId = "user-98765",
  commentId = "comment-0a1b2c3d",
  sso = "sso-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
)
if response.isSome:
  let badges = response.get()
  echo "Received manual badges for user"
  echo "HTTP status: ", httpResponse.status
[inline-code-end]