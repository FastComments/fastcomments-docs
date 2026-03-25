---
## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createTenantUserBody | CreateTenantUserBody | כן |  |

## תשובה

מחזיר: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6f4b2c';
const createTenantUserBody: CreateTenantUserBody = {
  email: 'sara.kim@example.com',
  displayName: 'Sara Kim',
  role: 'moderator',
  notifyOnMentions: true
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---