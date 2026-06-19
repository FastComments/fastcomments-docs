---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createTenantPackageBody | CreateTenantPackageBody | Oui |  |

## Réponse

Retourne: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = 'tenant_acme_01';
  const createTenantPackageBody: CreateTenantPackageBody = {
    packageName: 'Pro Annual',
    seats: 100,
    billingCycle: 'annual',
    autoRenew: true,
    metadata: { region: 'us-west-2' } // champ metadata optionnel
  };
  const result: CreateTenantPackageResponse = await createTenantPackage(tenantId, createTenantPackageBody);
  console.log(result);
}
run();
[inline-code-end]

---