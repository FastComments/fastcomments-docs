## 参数

| 名称 | 类型 | 必填 | 说明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 响应

返回: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsers200Response.ts)

## 示例

[inline-code-attrs-start title = 'getTenantUsers 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c1a';
const skip: number = 50;
const firstPage: GetTenantUsers200Response = await getTenantUsers(tenantId);
const nextPage: GetTenantUsers200Response = await getTenantUsers(tenantId, skip);
[inline-code-end]