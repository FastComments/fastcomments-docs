## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantUserBody | CreateTenantUserBody | Yes |  |

## Одговор

Враћа: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_74b3a9f4b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "jane.doe@acmecorp.com",
  displayName: "Jane Doe",
  role: "moderator",
  sendWelcomeEmail: true, // опциони параметар демонстриран
  metadata: { department: "Customer Support" }
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---