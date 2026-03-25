## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| skip | number | Не |  |

## Отговор

Връща: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsers200Response.ts)

## Пример

[inline-code-attrs-start title = 'getTenantUsers Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_8a3f2c';
const skip: number = 50;
const usersWithSkip: GetTenantUsers200Response = await getTenantUsers(tenantId, skip);
const usersNoSkip: GetTenantUsers200Response = await getTenantUsers(tenantId);
[inline-code-end]

---