req
tenantId
afterId

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| afterId | string | Нет |  |
| limit | int | Нет |  |
| tags | seq[string] | Нет |  |
| sso | string | Нет |  |
| isCrawler | bool | Нет |  |
| includeUserInfo | bool | Нет |  |

## Ответ

Возвращает: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример использования getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---