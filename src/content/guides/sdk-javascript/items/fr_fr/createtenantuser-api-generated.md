## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createTenantUserBody | CreateTenantUserBody | Oui |  |

## Réponse

Renvoie: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_74b3a9f4b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "jane.doe@acmecorp.com",
  displayName: "Jane Doe",
  role: "moderator",
  sendWelcomeEmail: true, // paramètre optionnel démontré
  metadata: { department: "Customer Support" }
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---