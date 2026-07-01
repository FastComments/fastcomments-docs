## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`GetCountsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCountsResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCounts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "acme-corp";
    const ssoToken: string = "sso-token-2024";

    const withBoth: GetCountsResponse = await getCounts(tenantId, ssoToken);
    const withTenantOnly: GetCountsResponse = await getCounts(tenantId);
    const withoutParams: GetCountsResponse = await getCounts();

    console.log(withBoth, withTenantOnly, withoutParams);
}
runExample();
[inline-code-end]

---