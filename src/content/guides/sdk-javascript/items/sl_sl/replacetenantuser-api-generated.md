## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Da |  |
| updateComments | string | Ne |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primer

[inline-code-attrs-start title = 'replaceTenantUser Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
const userId: string = "user-987654321";
const replaceBody: ReplaceTenantUserBody = {
  email: "newuser@example.com",
  name: "New User",
  role: "admin"
};

const response: APIEmptyResponse = await replaceTenantUser(
  tenantId,
  userId,
  replaceBody,
  "Migrated user to new tenant"
);
[inline-code-end]