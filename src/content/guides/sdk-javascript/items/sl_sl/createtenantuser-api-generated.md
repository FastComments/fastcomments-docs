## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createTenantUserBody | CreateTenantUserBody | Da |  |

## Odgovor

Vrača: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8c7e4b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "julia.smith@acme-corp.com",
  displayName: "Julia Smith",
  sendInviteEmail: true, // prikazan je opcijski parameter
  locale: "en-US",
  metadata: { department: "Customer Success" }
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---