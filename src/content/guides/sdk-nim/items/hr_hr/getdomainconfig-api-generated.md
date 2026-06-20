---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| domain | string | Ne |  |

## Odgovor

Vraća: [`Option[GetDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config_response.nim)

## Primjer

[inline-code-attrs-start title = 'getDomainConfig Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getDomainConfig(tenantId = "my-tenant-123", domain = "news/top-story-2026")
if response.isSome:
  let cfg = response.get()
  discard cfg
else:
  discard httpResponse
[inline-code-end]

---