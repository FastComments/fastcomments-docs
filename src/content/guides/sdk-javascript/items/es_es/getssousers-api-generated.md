## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| skip | number | No |  |

## Respuesta

Devuelve: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'getSSOUsers Ejemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
    const tenantId: string = "tenant_12345";

    // Llamada sin `skip` opcional
    const firstPage: GetSSOUsersResponse = await getSSOUsers(tenantId);

    // Llamada con `skip` opcional
    const secondPage: GetSSOUsersResponse = await getSSOUsers(tenantId, 100);

    console.log(firstPage, secondPage);
}

runExample();
[inline-code-end]