---
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackage200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_westus_01';
const packageId: string = 'pkg_premium_annual_2026';
interface FetchOptions { includeArchived?: boolean }
const options: FetchOptions = { includeArchived: false };
const result: GetTenantPackage200Response = await getTenantPackage(tenantId, packageId);
[inline-code-end]

---