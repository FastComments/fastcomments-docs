## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |

## Respons

Geeft terug: [`Option[GetVotesForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_for_user_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getVotesForUser Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getVotesForUser(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  userId = "user-789",
  anonUserId = ""
)
if response.isSome:
  let votes = response.get()
  echo "User votes retrieved"
else:
  echo "No votes found"
[inline-code-end]

---