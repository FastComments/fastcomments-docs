## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| createTenantBody | CreateTenantBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateTenantResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse1.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'createTenant Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = 'tenant-2024-01';
  const createTenantBody: CreateTenantBody = {
    // απαιτούμενα πεδία
    name: 'Acme International',
    // προαιρετικά πεδία μπορούν να προστεθούν ανάλογα, π.χ.:
    // billingInfo: { address: '123 Main St', city: 'Metropolis' } as BillingInfo,
  };
  const response: CreateTenantResponse1 = await createTenant(tenantId, createTenantBody);
  console.log(response);
}
[inline-code-end]