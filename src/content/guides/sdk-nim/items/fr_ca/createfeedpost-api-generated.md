## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| createFeedPostParams | CreateFeedPostParams | Non |  |
| options | CreateFeedPostOptions | Non |  |

## Réponse

Renvoie : [`Option[CreateFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_posts_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de createFeedPost'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.createFeedPost(
  tenantId = "my-tenant-123",
  createFeedPostParams = CreateFeedPostParams(),
  options = CreateFeedPostOptions()
)

if respOpt.isSome:
  let feedPost = respOpt.get()
[inline-code-end]