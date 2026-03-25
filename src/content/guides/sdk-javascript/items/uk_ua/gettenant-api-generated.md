## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenant200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f4b2c1a";
const idOverride: string | undefined = undefined; // необов'язкове перевизначення, якщо доступне
const id: string = idOverride ?? "site_3e7a6b2f";
const response: GetTenant200Response = await getTenant(tenantId, id);
console.log(response);
[inline-code-end]