---
## פרמטרים

| שם | סוג | מחובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| addDomainConfigParams | AddDomainConfigParams | לא |  |

## תגובה

מחזיר: [`Option[AddDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_domain_config_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת addDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.addDomainConfig(
  tenantId = "my-tenant-123",
  addDomainConfigParams = default(AddDomainConfigParams)
)

if respOpt.isSome:
  let cfg = respOpt.get()
[inline-code-end]

---