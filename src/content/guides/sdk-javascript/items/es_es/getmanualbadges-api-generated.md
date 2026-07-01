## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`GetManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetManualBadgesResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getManualBadges'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchBadges() {
  const tenantId: string = "tenant_987654321";
  const ssoToken: string = "sso_ABCdef123456";

  // Llamada con ambos parámetros opcionales
  const responseFull: GetManualBadgesResponse = await getManualBadges(tenantId, ssoToken);
  console.log(responseFull);

  // Llamada solo con tenantId
  const responseTenantOnly: GetManualBadgesResponse = await getManualBadges(tenantId);
  console.log(responseTenantOnly);
}

fetchBadges();
[inline-code-end]