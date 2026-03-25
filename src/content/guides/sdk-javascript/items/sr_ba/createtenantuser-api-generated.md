## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createTenantUserBody | CreateTenantUserBody | Да |  |

## Одговор

Враћа: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Примјер

[inline-code-attrs-start title = 'Примјер createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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