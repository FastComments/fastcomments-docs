## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createTenantUserBody | CreateTenantUserBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'createTenantUser Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8c7e4b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "julia.smith@acme-corp.com",
  displayName: "Julia Smith",
  sendInviteEmail: true, // προαιρετική παράμετρος για επίδειξη
  locale: "en-US",
  metadata: { department: "Customer Success" }
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---