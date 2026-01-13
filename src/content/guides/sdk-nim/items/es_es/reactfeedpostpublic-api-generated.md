## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| postId | string | No |  |
| reactBodyParams | ReactBodyParams | No |  |
| isUndo | bool | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[ReactFeedPostPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_public200response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de reactFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.reactFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "news/article-title",
  reactBodyParams = ReactBodyParams(),
  isUndo = false,
  broadcastId = "broadcast-456",
  sso = ""
)
if response.isSome:
  let result = response.get()
  echo "Reaction result: ", result
else:
  echo "Reaction failed, HTTP response: ", httpResponse
[inline-code-end]

---