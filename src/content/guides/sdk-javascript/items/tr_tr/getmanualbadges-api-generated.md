## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`GetManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetManualBadgesResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getManualBadges Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchBadges() {
  const tenantId: string = "tenant_987654321";
  const ssoToken: string = "sso_ABCdef123456";

  // Hem iki isteğe bağlı parametreyle çağır
  const responseFull: GetManualBadgesResponse = await getManualBadges(tenantId, ssoToken);
  console.log(responseFull);

  // Yalnızca tenantId ile çağır
  const responseTenantOnly: GetManualBadgesResponse = await getManualBadges(tenantId);
  console.log(responseTenantOnly);
}

fetchBadges();
[inline-code-end]