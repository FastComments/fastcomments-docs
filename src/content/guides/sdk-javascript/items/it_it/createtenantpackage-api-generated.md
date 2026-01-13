## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| createTenantPackageBody | CreateTenantPackageBody | Sì |  |

## Risposta

Restituisce: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b1a9c";
const tenantPackage: TenantPackage = { id: "pkg_001", name: "Premium Plan", seats: 100 };
const customConfig: CustomConfigParameters = { enableImages: true, maxImageSizeMb: 10 };
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: "Premium Plus",
  tenantPackage,
  customConfig,
  notes: "Enable advanced moderation and image uploads" // parametro opzionale dimostrativo
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]

---