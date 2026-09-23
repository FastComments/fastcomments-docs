## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| createTenantPackageBody | CreateTenantPackageBody | Ja |  |

## Antwort

Rückgabe: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'createTenantPackage Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9f8b7c6d";
  const createTenantPackageBody: CreateTenantPackageBody = {
    name: "Enterprise Package",
    priceCents: 49999,
    // optionales Feld
    description: "Full suite of enterprise features"
  };
  const result: CreateTenantPackageResponse = await createTenantPackage(tenantId, createTenantPackageBody);
  console.log(result);
}
runExample();
[inline-code-end]