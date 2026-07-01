---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantBody | CreateTenantBody | Yes |  |

## Svar

Returnerer: [`CreateTenantResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse1.ts)

## Eksempel

[inline-code-attrs-start title = 'createTenant Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = 'tenant-2024-01';
  const createTenantBody: CreateTenantBody = {
    // påkrævede felter
    name: 'Acme International',
    // valgfrie felter kan tilføjes efter behov, f.eks.:
    // billingInfo: { address: '123 Main St', city: 'Metropolis' } as BillingInfo,
  };
  const response: CreateTenantResponse1 = await createTenant(tenantId, createTenantBody);
  console.log(response);
}
[inline-code-end]

---