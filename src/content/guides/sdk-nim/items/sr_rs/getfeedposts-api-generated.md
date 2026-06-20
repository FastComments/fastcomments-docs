req
tenantId
afterId

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| afterId | string | Не |  |
| limit | int | Не |  |
| tags | seq[string] | Не |  |

## Одговор

Враћа: [`Option[GetFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_response.nim)

## Пример

[inline-code-attrs-start title = 'getFeedPosts Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPosts(
  tenantId = "my-tenant-123",
  afterId = "",
  limit = 0,
  tags = @[]
)
if response.isSome:
  let feed = response.get()
  echo "Feed retrieved for tenant my-tenant-123"
[inline-code-end]

---