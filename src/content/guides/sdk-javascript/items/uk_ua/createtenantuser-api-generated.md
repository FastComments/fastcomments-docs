---
## Parameters

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createTenantUserBody | CreateTenantUserBody | Так |  |

## Відповідь

Повертає: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Приклад

[inline-code-attrs-start title = 'createTenantUser Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6f4b2c';
const createTenantUserBody: CreateTenantUserBody = {
  email: 'sara.kim@example.com',
  displayName: 'Sara Kim',
  role: 'moderator',
  notifyOnMentions: true
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---