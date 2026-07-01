## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |

## Response

Retourneert: [`Option[GetDomainConfigsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_configs_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getDomainConfigs Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (domainConfigsOpt, httpResponse) = client.getDomainConfigs(tenantId = "my-tenant-123")
if domainConfigsOpt.isSome:
  let domainConfigs = domainConfigsOpt.get()
  echo domainConfigs
[inline-code-end]