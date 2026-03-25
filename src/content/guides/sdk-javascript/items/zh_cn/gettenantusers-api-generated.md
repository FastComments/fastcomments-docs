## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 响应

返回: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsers200Response.ts)

## 示例

[inline-code-attrs-start title = 'getTenantUsers 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_8a3f2c';
const skip: number = 50;
const usersWithSkip: GetTenantUsers200Response = await getTenantUsers(tenantId, skip);
const usersNoSkip: GetTenantUsers200Response = await getTenantUsers(tenantId);
[inline-code-end]

---