## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| addDomainConfigParams | AddDomainConfigParams | Ne |  |

## Odgovor

Vraća: [`Option[AddDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_domain_config200response.nim)

## Primjer

[inline-code-attrs-start title = 'addDomainConfig Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.addDomainConfig(tenantId = "my-tenant-123", addDomainConfigParams = AddDomainConfigParams())
if response.isSome:
  let domainConfig = response.get()
  discard domainConfig
else:
  discard httpResponse
[inline-code-end]