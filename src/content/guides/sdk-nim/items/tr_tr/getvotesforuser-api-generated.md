## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| userId | string | Hayır |  |
| anonUserId | string | Hayır |  |

## Yanıt

Döndürür: [`Option[GetVotesForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_for_user_response.nim)

## Örnek

[inline-code-attrs-start title = 'getVotesForUser Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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