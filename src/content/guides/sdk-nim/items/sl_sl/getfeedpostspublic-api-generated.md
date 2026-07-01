req
tenantId
afterId

## Parametri

| Ime | Tip | Potrebno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| options | GetFeedPostsPublicOptions | Ne |  |

## Odgovor

Vrača: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (feedResponseOpt, httpResponse) = client.getFeedPostsPublic(tenantId = "my-tenant-123", options = GetFeedPostsPublicOptions())
if feedResponseOpt.isSome:
  let feedResponse = feedResponseOpt.get()
[inline-code-end]

---