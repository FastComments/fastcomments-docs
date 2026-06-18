## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createTenantUserBody | CreateTenantUserBody | כן |  |

## תגובה

מחזיר: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f4a2b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "jane.doe@example.com",
  firstName: "Jane",
  lastName: "Doe",
  role: "commenter",
  approved: true,
  displayName: "Jane D." // אופציונלי: מתן שם ידידותי
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
console.log(result);
[inline-code-end]

---