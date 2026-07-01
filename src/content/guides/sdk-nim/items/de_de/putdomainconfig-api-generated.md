## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| domainToUpdate | string | Nein |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Nein |  |

## Antwort

Rückgabe: [`Option[PutDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_put_domain_config_response.nim)

## Beispiel

[inline-code-attrs-start title = 'putDomainConfig Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.putDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "example.com",
  updateDomainConfigParams = UpdateDomainConfigParams()
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]