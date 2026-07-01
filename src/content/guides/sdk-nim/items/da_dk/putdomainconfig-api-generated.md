## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| domainToUpdate | string | Nej |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Nej |  |

## Svar

Returnerer: [`Option[PutDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_put_domain_config_response.nim)

## Eksempel

[inline-code-attrs-start title = 'putDomainConfig Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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