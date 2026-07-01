## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| addDomainConfigParams | AddDomainConfigParams | Non |  |

## Réponse

Renvoie: [`Option[AddDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_domain_config_response.nim)

## Exemple

[inline-code-attrs-start title = 'addDomainConfig Exemple'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.addDomainConfig(
  tenantId = "my-tenant-123",
  addDomainConfigParams = default(AddDomainConfigParams)
)

if respOpt.isSome:
  let cfg = respOpt.get()
[inline-code-end]