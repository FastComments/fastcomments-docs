## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createTenantPackageBody | CreateTenantPackageBody | Da |  |

## Odgovor

Vrne: [`CreateTenantPackageResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse1.ts)

## Primer

[inline-code-attrs-start title = 'createTenantPackage Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9876";

  const body: CreateTenantPackageBody = {
    packageName: "Standard",
    quota: 5000,
    // neobvezno polje
    description: "Standard package for medium traffic",
  };

  const result: CreateTenantPackageResponse1 = await createTenantPackage(tenantId, body);
  console.log(result);
})();
[inline-code-end]

---