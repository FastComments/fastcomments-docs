[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

此路由提供新增單一個 `Tenant` 的功能。

建立 `Tenant` 有以下限制：

- 必須提供 `name`。
- 必須提供 `domainConfiguration`。
- 建立 `Tenant` 時不得提供下列值：
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- `signUpDate` 不得為未來時間。
- `name` 長度不得超過 `200 characters`。
- `email` 長度不得超過 `300 characters`。
- `email` 必須在所有 FastComments.com 的租戶中保持唯一。
- 若父租戶未定義有效的 `TenantPackage`，則不得建立租戶。
  - 如果您的租戶是透過 FastComments.com 建立的，這通常不會是問題。
- 您不得建立超過套件中 `maxWhiteLabeledTenants` 定義的租戶數量。
- 您必須指定 `tenantId` 查詢參數，此參數為啟用白牌（white labeling）的 `parent tenant` 的 id。

我們只需少數參數即可建立 `Tenant`：

[inline-code-attrs-start title = '建立 Tenant 的 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "domainConfiguration": [ { "domain": "somedomain.com" } ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant 建立請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant 建立回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** 失敗時包含。 **/
    reason?: string
    tenant?: Tenant; // 成功時會回傳完整的已建立租戶。
}
[inline-code-end]

---