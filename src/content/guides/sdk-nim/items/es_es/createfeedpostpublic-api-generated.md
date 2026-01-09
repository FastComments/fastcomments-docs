## Parámetros

| Name | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createFeedPostParams | CreateFeedPostParams | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[CreateFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_public200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createFeedPostPublic(
  tenantId = "my-tenant-123",
  createFeedPostParams = CreateFeedPostParams(
    title = "Product Launch Announcement",
    content = "We just launched a new commenting feature to improve engagement.",
    authorId = "team-product",
    url = "news/product-launch",
    tags = @["launch", "comments"],
    isFeatured = false
  ),
  broadcastId = "broadcast-009",
  sso = ""
)
if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]

---