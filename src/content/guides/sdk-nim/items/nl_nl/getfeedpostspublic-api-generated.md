req
tenantId
afterId

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| options | GetFeedPostsPublicOptions | Nee |  |

## Reactie

Retourneert: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (feedResponseOpt, httpResponse) = client.getFeedPostsPublic(tenantId = "my-tenant-123", options = GetFeedPostsPublicOptions())
if feedResponseOpt.isSome:
  let feedResponse = feedResponseOpt.get()
[inline-code-end]