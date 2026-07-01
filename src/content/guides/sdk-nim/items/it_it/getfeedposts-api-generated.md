req
tenantId
afterId

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| options | GetFeedPostsOptions | No |  |

## Risposta

Restituisce: [`Option[GetFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getFeedPosts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (feedResponseOpt, httpResp) = client.getFeedPosts(tenantId = "my-tenant-123", options = GetFeedPostsOptions())
if feedResponseOpt.isSome:
  let feedResponse = feedResponseOpt.get()
  echo feedResponse
[inline-code-end]