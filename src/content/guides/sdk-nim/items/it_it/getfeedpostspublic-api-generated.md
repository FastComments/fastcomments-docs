req
tenantId
afterId

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| options | GetFeedPostsPublicOptions | No |  |

## Risposta

Restituisce: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (feedResponseOpt, httpResponse) = client.getFeedPostsPublic(tenantId = "my-tenant-123", options = GetFeedPostsPublicOptions())
if feedResponseOpt.isSome:
  let feedResponse = feedResponseOpt.get()
[inline-code-end]