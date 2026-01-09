## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nee |  |
| limit | int | Nee |  |
| tags | seq[string] | Nee |  |
| sso | string | Nee |  |
| isCrawler | bool | Nee |  |
| includeUserInfo | bool | Nee |  |

## Antwoord

Retourneert: [`Option[GetFeedPostsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_public200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPostsPublic Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
  echo feed
[inline-code-end]