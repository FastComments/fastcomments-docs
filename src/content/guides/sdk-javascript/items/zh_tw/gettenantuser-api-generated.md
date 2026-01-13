## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## 範例

[inline-code-attrs-start title = 'getTenantUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f7d4b2a-1c3e";
const id: string = "user_6a12b3c4d5";
const includeProfile: boolean | undefined = true; // 可選參數範例
const response: GetTenantUser200Response = await getTenantUser(tenantId, id);
console.log("Tenant user fetched", response);
[inline-code-end]

---