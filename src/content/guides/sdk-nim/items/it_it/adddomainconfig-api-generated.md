## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| addDomainConfigParams | AddDomainConfigParams | No |  |

## Risposta

Restituisce: [`Option[AddDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_domain_config_response.nim)

## Esempio

[inline-code-attrs-start title = 'addDomainConfig Esempio'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.addDomainConfig(
  tenantId = "my-tenant-123",
  addDomainConfigParams = default(AddDomainConfigParams)
)

if respOpt.isSome:
  let cfg = respOpt.get()
[inline-code-end]