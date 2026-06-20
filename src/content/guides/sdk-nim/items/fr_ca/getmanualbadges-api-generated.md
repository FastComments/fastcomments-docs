## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| sso | string | Non |  |

## Réponse

Retourne : [`Option[GetTenantManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_manual_badges_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getManualBadges'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getManualBadges(sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")
if response.isSome:
  let badges = response.get()
  echo "Manual badges received:"
  echo badges
else:
  echo "No manual badges returned."
  echo httpResponse
[inline-code-end]

---