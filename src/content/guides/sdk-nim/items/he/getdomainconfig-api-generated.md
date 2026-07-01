## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | No |  |

## תגובה

מחזיר: [`Option[GetDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (configOpt, httpResp) = client.getDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if configOpt.isSome:
  let cfg = configOpt.get()
  discard cfg
[inline-code-end]