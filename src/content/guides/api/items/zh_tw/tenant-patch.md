[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

此 API 端點提供依 `id` 更新 `Tenant` 的功能。

更新 `Tenant` 有下列限制：

- 下列值不得更新：
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` 不能是未來的時間。
- `name` 的長度不得超過 `200 characters`。
- `email` 的長度不得超過 `300 characters`。
- `email` 必須在所有 FastComments.com 的租戶中唯一。
- 當將 `billingInfoValid` 設為 `true` 時，必須在同一請求中提供 `billingInfo`。
- 您不得更新與您自己的租戶關聯的 `packageId`。
- 您不得更新與您自己的租戶關聯的 `paymentFrequency`。

[inline-code-attrs-start title = '租戶 PATCH cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = '租戶 PATCH 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '租戶 PATCH 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** 失敗時包含。 **/
    reason?: string
}
[inline-code-end]

---