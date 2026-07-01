req
tenantId
afterId

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | GetFeedPostsPublicOptions | Ne |  |

## Odgovor

Vraća: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (feedResponseOpt, httpResponse) = client.getFeedPostsPublic(tenantId = "my-tenant-123", options = GetFeedPostsPublicOptions())
if feedResponseOpt.isSome:
  let feedResponse = feedResponseOpt.get()
[inline-code-end]