## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createTenantUserBody | CreateTenantUserBody | Так |  |

## Відповідь

Повертає: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f4a2b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "jane.doe@example.com",
  firstName: "Jane",
  lastName: "Doe",
  role: "commenter",
  approved: true,
  displayName: "Jane D." // необов'язково: вказати зручне ім'я
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
console.log(result);
[inline-code-end]