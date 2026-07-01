## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| broadcastId | string | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`UnPinCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnPinCommentResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unPinComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-001"
  const commentId: string = "comment-123"
  const broadcastId: string = "broadcast-456"
  const sso: string = "sso-token-xyz"

  const resultWithSso: UnPinCommentResponse = await unPinComment(tenantId, commentId, broadcastId, sso)
  const resultWithoutSso: UnPinCommentResponse = await unPinComment(tenantId, commentId, broadcastId)

  console.log(resultWithSso, resultWithoutSso)
})()
[inline-code-end]