req
tenantId
afterId

## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nej |  |
| limit | int | Nej |  |
| tags | seq[string] | Nej |  |
| sso | string | Nej |  |
| isCrawler | bool | Nej |  |
| includeUserInfo | bool | Nej |  |

## Respons

Returnerer: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getFeedPostsPublic Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsPublic(
  tenantId = "my-tenant-123",
  afterId = "",
  limit = 0,
  tags = @[],
  sso = "",
  isCrawler = false,
  includeUserInfo = false
)
if response.isSome:
  let feed = response.get()
  discard feed
[inline-code-end]