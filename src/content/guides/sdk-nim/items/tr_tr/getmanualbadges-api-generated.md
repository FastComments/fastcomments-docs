---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| sso | string = "" | Hayır |  |

## Yanıt

Döndürür: [`Option[GetTenantManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_manual_badges_response.nim)

## Örnek

[inline-code-attrs-start title = 'getManualBadges Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (manualBadgesOpt, httpResponse) = client.getManualBadges(tenantId = "my-tenant-123", sso = "")
if manualBadgesOpt.isSome:
  let manualBadges = manualBadgesOpt.get()
  echo manualBadges
[inline-code-end]

---