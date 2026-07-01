## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | No |  |

## Yanıt

Döndürür: [`Option[DeleteDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_domain_config_response.nim)

## Örnek

[inline-code-attrs-start title = 'deleteDomainConfig Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.deleteDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]