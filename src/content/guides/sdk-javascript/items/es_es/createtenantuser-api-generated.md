## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createTenantUserBody | CreateTenantUserBody | Sí |  |

## Respuesta

Devuelve: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f4a2b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "jane.doe@example.com",
  firstName: "Jane",
  lastName: "Doe",
  role: "commenter",
  approved: true,
  displayName: "Jane D." // opcional: proporcionar un nombre para mostrar
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
console.log(result);
[inline-code-end]

---