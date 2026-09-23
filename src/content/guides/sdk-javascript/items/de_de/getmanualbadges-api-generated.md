## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantManualBadgesResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getManualBadges Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchBadges() {
  const tenantId: string = "tenant-987654321";
  const ssoToken: string = "sso-token-abc123";

  const badgesWithSso: GetTenantManualBadgesResponse = await getManualBadges(tenantId, ssoToken);
  const badgesWithoutSso: GetTenantManualBadgesResponse = await getManualBadges(tenantId);
}
[inline-code-end]

---