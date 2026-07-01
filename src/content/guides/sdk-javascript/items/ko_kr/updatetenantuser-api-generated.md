## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateTenantUserBody | UpdateTenantUserBody | 예 |  |
| updateComments | string | 아니오 |  |

## 응답

반환: [`UpdateTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateTenantUserResponse.ts)

## 예시

[inline-code-attrs-start title = 'updateTenantUser 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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