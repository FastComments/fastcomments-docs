## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createTenantPackageBody | CreateTenantPackageBody | Так |  |

## Відповідь

Повертає: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b1a9c";
const tenantPackage: TenantPackage = { id: "pkg_001", name: "Premium Plan", seats: 100 };
const customConfig: CustomConfigParameters = { enableImages: true, maxImageSizeMb: 10 };
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: "Premium Plus",
  tenantPackage,
  customConfig,
  notes: "Enable advanced moderation and image uploads" // необов'язковий параметр, продемонстрований
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]