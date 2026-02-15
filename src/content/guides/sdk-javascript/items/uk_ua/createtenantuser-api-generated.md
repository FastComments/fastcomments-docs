## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantUserBody | CreateTenantUserBody | Yes |  |

## Відповідь

Повертає: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8c7e4b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "julia.smith@acme-corp.com",
  displayName: "Julia Smith",
  sendInviteEmail: true, // показано необов'язковий параметр
  locale: "en-US",
  metadata: { department: "Customer Success" }
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---