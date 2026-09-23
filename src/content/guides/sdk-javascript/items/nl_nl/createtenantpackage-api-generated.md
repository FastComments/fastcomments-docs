## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| createTenantPackageBody | CreateTenantPackageBody | Ja |  |

## Respons

Retourneert: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createTenantPackage Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9f8b7c6d";
  const createTenantPackageBody: CreateTenantPackageBody = {
    name: "Enterprise Package",
    priceCents: 49999,
    // optioneel veld
    description: "Full suite of enterprise features"
  };
  const result: CreateTenantPackageResponse = await createTenantPackage(tenantId, createTenantPackageBody);
  console.log(result);
}
runExample();
[inline-code-end]