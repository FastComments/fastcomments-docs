[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

此 API 端点提供通过 `id` 更新 `TenantPackage` 的功能。

更新 `TenantPackage` 有以下限制：

- 如果将 `hasFlexPricing` 设置为 true，则同一请求中必须包含所有 `flex*` 参数。
- `name` 不得超过 `50 characters`。
- 每个 `forWhoText` 项的长度不得超过 `200 characters`。
- 每个 `featureTaglines` 项的长度不得超过 `100 characters`。
- `TenantPackage` 必须比父租户“更小”。例如，所有 `max*` 参数的值必须小于父租户的对应值。
- 不得更改与 `TenantPackage` 关联的 `tenantId`。

[inline-code-attrs-start title = 'TenantPackage PATCH cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** 失败时包含。 **/
    reason?: string
}
[inline-code-end]

---