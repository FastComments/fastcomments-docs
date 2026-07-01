## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`DeleteUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteUserBadgeResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'deleteUserBadge Ejemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleDeleteBadge(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6a";
  const badgeId: string = "badge_4e3d2c1b";
  const result: DeleteUserBadgeResponse = await deleteUserBadge(tenantId, badgeId);
  console.log(result);
}
exampleDeleteBadge();
[inline-code-end]