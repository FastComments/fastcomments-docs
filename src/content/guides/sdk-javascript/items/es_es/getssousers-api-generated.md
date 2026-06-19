## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| skip | number | No |  |

## Respuesta

Devuelve: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getSSOUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f3b2a1c";
const usersWithoutSkip: GetSSOUsersResponse = await getSSOUsers(tenantId);
const skip: number = 50;
const usersWithSkip: GetSSOUsersResponse = await getSSOUsers(tenantId, skip);
[inline-code-end]

---