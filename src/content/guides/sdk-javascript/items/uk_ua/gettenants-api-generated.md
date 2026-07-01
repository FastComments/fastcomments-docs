## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| meta | string | No |  |
| skip | number | No |  |

## Відповідь

Повертає: [`GetTenantsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantsResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getTenants'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9f8e7d6c";
  const resultOnlyId: GetTenantsResponse1 = await getTenants(tenantId);
  const resultWithMeta: GetTenantsResponse1 = await getTenants(tenantId, "full");
  const resultAllParams: GetTenantsResponse1 = await getTenants(tenantId, "full", 15);
  console.log(resultOnlyId, resultWithMeta, resultAllParams);
})();
[inline-code-end]