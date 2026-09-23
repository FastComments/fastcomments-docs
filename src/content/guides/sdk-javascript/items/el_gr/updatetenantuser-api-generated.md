## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantUserBody | UpdateTenantUserBody | Yes |  |
| updateComments | string | No |  |

## Απάντηση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_987";
  const updateBody: UpdateTenantUserBody = {
    email: "new.email@example.com",
    role: "admin",
    isActive: true
  };
  const comment: string = "Promoted to admin role";

  const result: APIEmptyResponse = await updateTenantUser(tenantId, userId, updateBody, comment);
  console.log(result);
}

runUpdate();
[inline-code-end]