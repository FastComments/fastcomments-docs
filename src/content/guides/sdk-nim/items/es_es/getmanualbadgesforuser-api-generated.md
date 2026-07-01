## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| options | GetManualBadgesForUserOptions | No |  |

## Respuesta

Devuelve: [`Option[GetUserManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_manual_badges_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'getManualBadgesForUser Ejemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (userBadgesOpt, httpResp) = client.getManualBadgesForUser(
  tenantId = "my-tenant-123",
  options = GetManualBadgesForUserOptions()
)
if userBadgesOpt.isSome:
  let badges = userBadgesOpt.get()
  echo badges
[inline-code-end]

---