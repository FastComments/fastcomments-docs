## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateTenantUserBody | UpdateTenantUserBody | Sí |  |
| updateComments | string | No |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'updateTenantUser Ejemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_987";
  const updateBody: UpdateTenantUserBody = {
    email: "new.email@example.com",
    role: "admin",
    isActive: true
  };
  const comment: string = "Promoted to admin role";

  const result: APIEmptyResponse = await updateTenantUser(tenantId, userId, updateBody, comment);
  console.log(result);
}

runUpdate();
[inline-code-end]