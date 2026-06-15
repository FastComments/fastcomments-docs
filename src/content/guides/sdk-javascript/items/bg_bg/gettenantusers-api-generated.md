## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| skip | number | Не |  |

## Отговор

Връща: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsers200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getTenantUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b8f3a2c-9e4d-4f1a';
const skip: number = 50;
const usersResponseDefault: GetTenantUsers200Response = await getTenantUsers(tenantId);
const usersResponsePaged: GetTenantUsers200Response = await getTenantUsers(tenantId, skip);
[inline-code-end]