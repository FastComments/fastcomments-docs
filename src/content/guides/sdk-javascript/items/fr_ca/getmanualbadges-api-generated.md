## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`GetManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetManualBadgesResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getManualBadges'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchBadges() {
  const tenantId: string = "tenant_987654321";
  const ssoToken: string = "sso_ABCdef123456";

  // Appeler avec les deux paramètres optionnels
  const responseFull: GetManualBadgesResponse = await getManualBadges(tenantId, ssoToken);
  console.log(responseFull);

  // Appeler avec seulement tenantId
  const responseTenantOnly: GetManualBadgesResponse = await getManualBadges(tenantId);
  console.log(responseTenantOnly);
}

fetchBadges();
[inline-code-end]