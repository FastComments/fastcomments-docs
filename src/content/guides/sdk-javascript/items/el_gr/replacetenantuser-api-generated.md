---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Ναι |  |
| updateComments | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα replaceTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "f3b9a2d1-8b4e-4c6a-9f2b-1d5c4e6a7b8c";
const id: string = "user_92f7c3b1";
const replaceTenantUserBody: ReplaceTenantUserBody = {
  externalId: "auth0|1234567890",
  email: "jane.doe@company.com",
  displayName: "Jane Doe",
  roles: ["moderator"],
  metadata: { department: "support" }
};
const updateComments: string = "reassign-comments-to-new-user";
const response: FlagCommentPublic200Response = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]

---