## 參數

| 名稱 | 類型 | 是否必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantUserBody | CreateTenantUserBody | 是 |  |

## 回應

回傳：[`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## 範例

[inline-code-attrs-start title = 'createTenantUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f4a2b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "jane.doe@example.com",
  firstName: "Jane",
  lastName: "Doe",
  role: "commenter",
  approved: true,
  displayName: "Jane D." // 可選：提供友好顯示名稱
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
console.log(result);
[inline-code-end]

---