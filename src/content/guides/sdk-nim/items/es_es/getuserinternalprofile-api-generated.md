## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| options | GetUserInternalProfileOptions | No |  |

## Respuesta

Devuelve: [`Option[GetUserInternalProfileResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_internal_profile_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getUserInternalProfile'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (profileOpt, httpResp) = client.getUserInternalProfile(
  tenantId = "my-tenant-123",
  options = GetUserInternalProfileOptions()
)

if profileOpt.isSome:
  let profile = profileOpt.get()
[inline-code-end]