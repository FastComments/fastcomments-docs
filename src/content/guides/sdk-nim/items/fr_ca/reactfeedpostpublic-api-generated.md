## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| postId | string | Non |  |
| reactBodyParams | ReactBodyParams | Non |  |
| options | ReactFeedPostPublicOptions | Non |  |

## Réponse

Retourne : [`Option[ReactFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple reactFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let reactParams = ReactBodyParams()
let (optResp, httpResp) = client.reactFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-456",
  reactBodyParams = reactParams,
  options = ReactFeedPostPublicOptions()
)
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]