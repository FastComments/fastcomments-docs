## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createTenantPackageBody | CreateTenantPackageBody | Oui |  |

## Réponse

Retourne : [`CreateTenantPackageResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9876";

  const body: CreateTenantPackageBody = {
    packageName: "Standard",
    quota: 5000,
    // champ facultatif
    description: "Standard package for medium traffic",
  };

  const result: CreateTenantPackageResponse1 = await createTenantPackage(tenantId, body);
  console.log(result);
})();
[inline-code-end]