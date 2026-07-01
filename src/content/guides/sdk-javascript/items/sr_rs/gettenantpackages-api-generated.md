## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Одговор

Враћа: [`GetTenantPackagesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Primer getTenantPackages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_67890';
  const skip: number = 30;

  const packagesWithSkip: GetTenantPackagesResponse1 = await getTenantPackages(tenantId, skip);
  const packagesWithoutSkip: GetTenantPackagesResponse1 = await getTenantPackages(tenantId);

  console.log(packagesWithSkip, packagesWithoutSkip);
})();
[inline-code-end]

---