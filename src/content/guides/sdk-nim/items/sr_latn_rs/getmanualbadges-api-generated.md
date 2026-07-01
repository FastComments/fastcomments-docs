## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| sso | string = "" | Ne |  |

## Odgovor

Returns: [`Option[GetTenantManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_manual_badges_response.nim)

## Primer

[inline-code-attrs-start title = 'getManualBadges Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (manualBadgesOpt, httpResponse) = client.getManualBadges(tenantId = "my-tenant-123", sso = "")
if manualBadgesOpt.isSome:
  let manualBadges = manualBadgesOpt.get()
  echo manualBadges
[inline-code-end]