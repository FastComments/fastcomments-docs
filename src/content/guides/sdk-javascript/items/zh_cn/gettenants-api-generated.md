## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| meta | string | No |  |
| skip | number | No |  |

## 响应

返回: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## 示例

[inline-code-attrs-start title = 'getTenants 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fcom-tenant-8b4f2a1c";
const meta: string = "include=domains,billing&status=active";
const skip: number = 20;
const response: GetTenants200Response = await getTenants(tenantId, meta, skip);
console.log(response);
[inline-code-end]

---