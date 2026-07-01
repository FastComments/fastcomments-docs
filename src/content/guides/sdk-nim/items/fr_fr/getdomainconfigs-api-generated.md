## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Yes |  |

## Réponse

Renvoie : [`Option[GetDomainConfigsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_configs_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getDomainConfigs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (domainConfigsOpt, httpResponse) = client.getDomainConfigs(tenantId = "my-tenant-123")
if domainConfigsOpt.isSome:
  let domainConfigs = domainConfigsOpt.get()
  echo domainConfigs
[inline-code-end]