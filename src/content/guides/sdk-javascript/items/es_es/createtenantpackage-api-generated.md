## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createTenantPackageBody | CreateTenantPackageBody | Sí |  |

## Respuesta

Devuelve: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function createTenantPackageExample(): Promise<void> {
  const tenantId: string = '5f2a9b3c8e4d1a2b3c4d5e6f';

  const createTenantPackageBody: CreateTenantPackageBody = {
    name: 'Premium Support Package',
    packageId: 'tenant-premium-annual',
    billingInterval: 'yearly',
    price: 499.99,
    currency: 'USD',
    maxSeats: 25,
    status: 'active'
  };

  const result: CreateTenantPackageResponse = await createTenantPackage(
    tenantId,
    createTenantPackageBody
  );

  const status: APIStatus = result.status;
  const createdPackage: TenantPackage | undefined = result.tenantPackage;

  if (createdPackage) {
    console.log(`Created package ${createdPackage.name} with status ${status}`);
  }
}
[inline-code-end]