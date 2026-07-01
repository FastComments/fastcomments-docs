## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createTenantBody | CreateTenantBody | Da |  |

## Odgovor

Vraća: [`CreateTenantResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse1.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer createTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = 'tenant-2024-01';
  const createTenantBody: CreateTenantBody = {
    // obavezna polja
    name: 'Acme International',
    // opcionalna polja mogu se dodati po potrebi, npr.:
    // billingInfo: { address: '123 Main St', city: 'Metropolis' } as BillingInfo,
  };
  const response: CreateTenantResponse1 = await createTenant(tenantId, createTenantBody);
  console.log(response);
}
[inline-code-end]