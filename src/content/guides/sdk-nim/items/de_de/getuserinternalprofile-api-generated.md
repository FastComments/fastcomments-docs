## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| options | GetUserInternalProfileOptions | Nein |  |

## Antwort

Rückgabe: [`Option[GetUserInternalProfileResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_internal_profile_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getUserInternalProfile Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (profileOpt, httpResp) = client.getUserInternalProfile(
  tenantId = "my-tenant-123",
  options = GetUserInternalProfileOptions()
)

if profileOpt.isSome:
  let profile = profileOpt.get()
[inline-code-end]