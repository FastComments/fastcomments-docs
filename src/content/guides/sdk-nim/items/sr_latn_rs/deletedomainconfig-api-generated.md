## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| domain | string | Ne |  |

## Odgovor

Vraća: [`Option[DeleteDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_domain_config_response.nim)

## Primer

[inline-code-attrs-start title = 'deleteDomainConfig Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.deleteDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]