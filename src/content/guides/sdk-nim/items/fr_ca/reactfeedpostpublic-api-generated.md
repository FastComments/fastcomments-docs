## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| postId | string | Non |  |
| reactBodyParams | ReactBodyParams | Non |  |
| isUndo | bool | Non |  |
| broadcastId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`Option[ReactFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_public200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de reactFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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