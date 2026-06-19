## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Sí |  |
| updateComments | string | No |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de replaceTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-92";
const id: string = "user_7f9b2a";
const replaceTenantUserBody: ReplaceTenantUserBody = {
  email: "maria.garcia@acme-corp.com",
  displayName: "María García",
  role: "moderator",
  externalId: "ext-5271"
};
const updateComments: string = "true";
const result: APIEmptyResponse = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]

---