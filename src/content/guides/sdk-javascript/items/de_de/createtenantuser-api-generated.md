## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createTenantUserBody | CreateTenantUserBody | Ja |  |

## Antwort

Gibt zurück: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'createTenantUser Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_74b3a9f4b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "jane.doe@acmecorp.com",
  displayName: "Jane Doe",
  role: "moderator",
  sendWelcomeEmail: true, // optionaler Parameter demonstriert
  metadata: { department: "Customer Support" }
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---