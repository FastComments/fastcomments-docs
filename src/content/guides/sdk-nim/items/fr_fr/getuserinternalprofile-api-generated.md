## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| options | GetUserInternalProfileOptions | Non |  |

## Réponse

Retourne : [`Option[GetUserInternalProfileResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_internal_profile_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getUserInternalProfile'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (profileOpt, httpResp) = client.getUserInternalProfile(
  tenantId = "my-tenant-123",
  options = GetUserInternalProfileOptions()
)

if profileOpt.isSome:
  let profile = profileOpt.get()
[inline-code-end]