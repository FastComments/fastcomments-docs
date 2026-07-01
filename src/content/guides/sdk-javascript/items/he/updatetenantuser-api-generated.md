## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantUserBody | UpdateTenantUserBody | Yes |  |
| updateComments | string | No |  |

## תגובה

מחזיר: [`UpdateTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateTenantUserResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת updateTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId: string = "5f8f8c1a2e9b3c001c2a9b2d";
let userId: string = "user_98765";

let updateBody: UpdateTenantUserBody = {
  email: "jane.smith@example.com",
  role: "moderator",
  isActive: false,
};

let updateComments: string = "Deactivated user due to policy violation.";

let result: UpdateTenantUserResponse = await updateTenantUser(tenantId, userId, updateBody, updateComments);
[inline-code-end]