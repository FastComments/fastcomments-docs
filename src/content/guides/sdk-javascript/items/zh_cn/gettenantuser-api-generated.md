---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## 示例

[inline-code-attrs-start title = 'getTenantUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_ab12c3';
const id: string = 'user_9f8e7d';
const response: GetTenantUser200Response = await getTenantUser(tenantId, id);
console.log(response);
[inline-code-end]

---