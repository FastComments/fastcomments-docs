## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createTenantUserBody | CreateTenantUserBody | Да |  |

## Отговор

Връща: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8c7e4b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "julia.smith@acme-corp.com",
  displayName: "Julia Smith",
  sendInviteEmail: true, // демонстриран незадължителен параметър
  locale: "en-US",
  metadata: { department: "Customer Success" }
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---