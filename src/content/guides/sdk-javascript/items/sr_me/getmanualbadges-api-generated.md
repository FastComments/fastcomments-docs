## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`GetManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetManualBadgesResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getManualBadges'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchBadges() {
  const tenantId: string = "tenant_987654321";
  const ssoToken: string = "sso_ABCdef123456";

  // Poziv sa oba opcionalna parametra
  const responseFull: GetManualBadgesResponse = await getManualBadges(tenantId, ssoToken);
  console.log(responseFull);

  // Poziv samo sa tenantId
  const responseTenantOnly: GetManualBadgesResponse = await getManualBadges(tenantId);
  console.log(responseTenantOnly);
}

fetchBadges();
[inline-code-end]