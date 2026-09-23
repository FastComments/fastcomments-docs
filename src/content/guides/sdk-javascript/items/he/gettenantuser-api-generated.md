---
## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## תגובה

מחזיר: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUserResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-tenant-001";
    const userId: string = "user-12345";
    const response: GetTenantUserResponse = await getTenantUser(tenantId, userId);
    console.log(response);
})();
[inline-code-end]

---