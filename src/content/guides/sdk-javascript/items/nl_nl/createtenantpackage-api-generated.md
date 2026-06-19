---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createTenantPackageBody | CreateTenantPackageBody | Ja |  |

## Respons

Retourneert: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createTenantPackage Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = 'tenant_acme_01';
  const createTenantPackageBody: CreateTenantPackageBody = {
    packageName: 'Pro Annual',
    seats: 100,
    billingCycle: 'annual',
    autoRenew: true,
    metadata: { region: 'us-west-2' } // optioneel metadata-veld
  };
  const result: CreateTenantPackageResponse = await createTenantPackage(tenantId, createTenantPackageBody);
  console.log(result);
}
run();
[inline-code-end]

---