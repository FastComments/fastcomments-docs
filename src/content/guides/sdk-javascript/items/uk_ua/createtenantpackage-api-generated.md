## Parameters

| Назва | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenantId | string | Так |  |
| createTenantPackageBody | CreateTenantPackageBody | Так |  |

## Response

Повертає: [`CreateTenantPackageResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'createTenantPackage Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9876";

  const body: CreateTenantPackageBody = {
    packageName: "Standard",
    quota: 5000,
    // optional field
    description: "Standard package for medium traffic",
  };

  const result: CreateTenantPackageResponse1 = await createTenantPackage(tenantId, body);
  console.log(result);
})();
[inline-code-end]