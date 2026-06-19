## Параметри

| Назив | Type | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| meta | string | Не |  |
| skip | number | Не |  |

## Одговор

Враћа: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantsResponse.ts)

## Пример

[inline-code-attrs-start title = 'getTenants Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-72b';
const meta: string = 'include=domains,billing';
const skip: number = 20;
const result: GetTenantsResponse = await getTenants(tenantId, meta, skip);
[inline-code-end]