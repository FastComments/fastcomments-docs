req
tenantId
afterId

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| afterId | string | Non |  |
| limit | int | Non |  |
| tags | seq[string] | Non |  |

## Réponse

Renvoie: [`Option[GetFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getFeedPosts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPosts(
  tenantId = "my-tenant-123",
  afterId = "",
  limit = 0,
  tags = @[]
)
if response.isSome:
  let feed = response.get()
  echo "Feed retrieved for tenant my-tenant-123"
[inline-code-end]

---