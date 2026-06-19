## Parameters

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| id | string | Sí |  |

## Response

Devuelve: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactUsersResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getV2PageReactUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fc_tenant_7b4c9d1";
const rawUrlId: string | undefined = undefined; // may come from route params
const urlId: string = rawUrlId ?? "page-home-9a3f2b";
const id: string = "user_823b5c";

const response: GetV2PageReactUsersResponse = await getV2PageReactUsers(tenantId, urlId, id);
[inline-code-end]

---