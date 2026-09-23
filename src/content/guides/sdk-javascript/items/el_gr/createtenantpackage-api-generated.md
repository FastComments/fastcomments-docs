## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| createTenantPackageBody | CreateTenantPackageBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'createTenantPackage Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9f8b7c6d";
  const createTenantPackageBody: CreateTenantPackageBody = {
    name: "Enterprise Package",
    priceCents: 49999,
    // προαιρετικό πεδίο
    description: "Full suite of enterprise features"
  };
  const result: CreateTenantPackageResponse = await createTenantPackage(tenantId, createTenantPackageBody);
  console.log(result);
}
runExample();
[inline-code-end]