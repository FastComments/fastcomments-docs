## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postId | string | Nee |  |
| reactBodyParams | ReactBodyParams | Nee |  |
| isUndo | bool | Nee |  |
| broadcastId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Geeft terug: [`Option[ReactFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_public200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'reactFeedPostPublic Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.reactFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "news/article-title",
  reactBodyParams = ReactBodyParams(),
  isUndo = false,
  broadcastId = "broadcast-456",
  sso = ""
)
if response.isSome:
  let result = response.get()
  echo "Reaction result: ", result
else:
  echo "Reaction failed, HTTP response: ", httpResponse
[inline-code-end]

---