req
tenantId
afterId

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| afterId | string | Ne |  |
| limit | int | Ne |  |
| tags | seq[string] | Ne |  |

## Odziv

Vrne: [`Option[GetFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getFeedPosts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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