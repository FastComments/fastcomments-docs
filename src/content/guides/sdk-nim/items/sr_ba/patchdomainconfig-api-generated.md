## Parametri

| Naziv | Tip | Potrebno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| domainToUpdate | string | Ne |  |
| patchDomainConfigParams | PatchDomainConfigParams | Ne |  |

## Odgovor

Vraća: [`Option[PatchDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_domain_config_response.nim)

## Primer

[inline-code-attrs-start title = 'patchDomainConfig Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.patchDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "news.mywebsite.com",
  patchDomainConfigParams = PatchDomainConfigParams()
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]