## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Respuesta

Devuelve: [`GetTenantUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUserResponse1.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUser(): Promise<void> {
  const tenantId: string = "tenant-01a2b3c";
  const userId: string = "user-7890";
  const result: GetTenantUserResponse1 = await getTenantUser(tenantId, userId);
  console.log(result);
}
[inline-code-end]