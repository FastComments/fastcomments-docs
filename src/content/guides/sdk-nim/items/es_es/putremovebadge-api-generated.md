## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| badgeId | string | No |  |
| options | PutRemoveBadgeOptions | No |  |

## Respuesta

Devuelve: [`Option[RemoveUserBadgeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_remove_user_badge_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'putRemoveBadge Ejemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.putRemoveBadge(
  tenantId = "my-tenant-123",
  badgeId = "badge-456",
  options = PutRemoveBadgeOptions()
)

if maybeResp.isSome:
  let badgeResp = maybeResp.get()
[inline-code-end]