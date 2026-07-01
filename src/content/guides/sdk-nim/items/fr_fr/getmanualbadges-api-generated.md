## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string = "" | No |  |

## Réponse

Retourne : [`Option[GetTenantManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_manual_badges_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getManualBadges'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (manualBadgesOpt, httpResponse) = client.getManualBadges(tenantId = "my-tenant-123", sso = "")
if manualBadgesOpt.isSome:
  let manualBadges = manualBadgesOpt.get()
  echo manualBadges
[inline-code-end]

---