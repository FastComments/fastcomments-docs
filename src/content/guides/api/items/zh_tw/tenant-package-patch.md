[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

此 API 端點可用於透過 `id` 更新 `TenantPackage`。

更新 `TenantPackage` 有下列限制：

- 如果您將 `hasFlexPricing` 設為 true，則同一請求中必須包含所有 `flex*` 參數。
- `name` 可能不得超過 `50 characters`。
- 每個 `forWhoText` 項目可能不得超過 `200 characters`。
- 每個 `featureTaglines` 項目可能不得超過 `100 characters`。
- `TenantPackage` 必須比父租戶「小」。例如，所有的 `max*` 參數必須比父租戶的值更低。
- 您不得變更與 `TenantPackage` 關聯的 `tenantId`。

[inline-code-attrs-start title = 'TenantPackage PATCH cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage PATCH 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** 於失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** 於失敗時包含。 **/
    reason?: string
}
[inline-code-end]

---