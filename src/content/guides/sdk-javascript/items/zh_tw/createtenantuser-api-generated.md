## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantUserBody | CreateTenantUserBody | 是 |  |

## 回應

傳回: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## 範例

[inline-code-attrs-start title = 'createTenantUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_74b3a9f4b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "jane.doe@acmecorp.com",
  displayName: "Jane Doe",
  role: "moderator",
  sendWelcomeEmail: true, // 示範可選參數
  metadata: { department: "Customer Support" }
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]