## Paramètres

| Nom | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| createTenantBody | CreateTenantBody | Oui |  |

## Réponse

Renvoie : [`CreateTenantResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple createTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = 'tenant-2024-01';
  const createTenantBody: CreateTenantBody = {
    // champs requis
    name: 'Acme International',
    // les champs optionnels peuvent être ajoutés au besoin, p. ex. :
    // billingInfo: { address: '123 Main St', city: 'Metropolis' } as BillingInfo,
  };
  const response: CreateTenantResponse1 = await createTenant(tenantId, createTenantBody);
  console.log(response);
}
[inline-code-end]