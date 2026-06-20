req
tenantId
afterId

## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| afterId | string | Ні |  |
| limit | int | Ні |  |
| tags | seq[string] | Ні |  |
| sso | string | Ні |  |
| isCrawler | bool | Ні |  |
| includeUserInfo | bool | Ні |  |

## Відповідь

Повертає: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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