## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|------------|-------------|
| tenantId | string | Sí |  |
| createTenantBody | CreateTenantBody | Sí |  |

## Respuesta

Devuelve: [`CreateTenantResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse1.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo createTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = 'tenant-2024-01';
  const createTenantBody: CreateTenantBody = {
    // campos requeridos
    name: 'Acme International',
    // los campos opcionales pueden añadirse según sea necesario, p. ej.:
    // billingInfo: { address: '123 Main St', city: 'Metropolis' } as BillingInfo,
  };
  const response: CreateTenantResponse1 = await createTenant(tenantId, createTenantBody);
  console.log(response);
}
[inline-code-end]