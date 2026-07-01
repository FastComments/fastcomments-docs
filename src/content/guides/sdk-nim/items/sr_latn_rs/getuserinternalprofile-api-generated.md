## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| options | GetUserInternalProfileOptions | Ne |  |

## Odgovor

Vraća: [`Option[GetUserInternalProfileResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_internal_profile_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getUserInternalProfile'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (profileOpt, httpResp) = client.getUserInternalProfile(
  tenantId = "my-tenant-123",
  options = GetUserInternalProfileOptions()
)

if profileOpt.isSome:
  let profile = profileOpt.get()
[inline-code-end]