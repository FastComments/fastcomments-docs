## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| postId | string | No |  |
| updateFeedPostParams | UpdateFeedPostParams | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[CreateFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_public200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
var updateParams: UpdateFeedPostParams = UpdateFeedPostParams(
  title = "Breaking: Service Update",
  content = "We improved feed performance and UX for all users.",
  tags = @["performance", "release"],
  isPublic = true
)

let (response, httpResponse) = client.updateFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-456",
  updateFeedPostParams = updateParams,
  broadcastId = "broadcast-789",
  sso = "sso-token-abc123"
)

if response.isSome:
  let post = response.get()
  echo "Updated post title: ", post.title
  echo "HTTP status: ", httpResponse.status
[inline-code-end]

---