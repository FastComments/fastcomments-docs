## Parameter

| Name | Type | Required | Beschreibung |
|------|------|----------|----------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | Yes |  |

## Antwort

Rückgabe: [`CreateTenantPackageResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse1.ts)

## Beispiel

[inline-code-attrs-start title = 'createTenantPackage Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9876";

  const body: CreateTenantPackageBody = {
    packageName: "Standard",
    quota: 5000,
    // optional field -> optionales Feld
    description: "Standard package for medium traffic",
  };

  const result: CreateTenantPackageResponse1 = await createTenantPackage(tenantId, body);
  console.log(result);
})();
[inline-code-end]