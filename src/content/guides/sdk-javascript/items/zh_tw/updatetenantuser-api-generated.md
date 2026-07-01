## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateTenantUserBody | UpdateTenantUserBody | 是 |  |
| updateComments | string | 否 |  |

## 回應

返回: [`UpdateTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateTenantUserResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateTenantUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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