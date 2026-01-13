## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| createTenantPackageBody | CreateTenantPackageBody | Tak |  |

## Odpowiedź

Zwraca: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b1a9c";
const tenantPackage: TenantPackage = { id: "pkg_001", name: "Premium Plan", seats: 100 };
const customConfig: CustomConfigParameters = { enableImages: true, maxImageSizeMb: 10 };
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: "Premium Plus",
  tenantPackage,
  customConfig,
  notes: "Enable advanced moderation and image uploads" // parametr opcjonalny — przykład
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]