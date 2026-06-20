req
tenantId
afterId

## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| afterId | string | No |  |
| limit | int | No |  |
| tags | seq[string] | No |  |

## Respuesta

Devuelve: [`Option[GetFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getFeedPosts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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