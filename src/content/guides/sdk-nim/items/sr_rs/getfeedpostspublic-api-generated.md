req
tenantId
afterId

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| afterId | string | Не |  |
| limit | int | Не |  |
| tags | seq[string] | Не |  |
| sso | string | Не |  |
| isCrawler | bool | Не |  |
| includeUserInfo | bool | Не |  |

## Одговор

Враћа: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## Пример

[inline-code-attrs-start title = 'getFeedPostsPublic пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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