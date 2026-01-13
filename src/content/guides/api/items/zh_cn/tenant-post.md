[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

此路由允许添加单个 `Tenant`。

创建 `Tenant` 有以下限制：

- 需要提供 `name`。
- 需要提供 `domainConfiguration`。
- 创建 `Tenant` 时不得提供以下值：
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- `signUpDate` 不能是未来的日期。
- `name` 的长度不得超过 `200 characters`。
- `email` 的长度不得超过 `300 characters`。
- `email` 必须在所有 FastComments.com 的租户中唯一。
- 如果父租户未定义有效的 `TenantPackage`，则您不得创建租户。
  - 如果您的租户是通过 FastComments.com 创建的，通常不会有此问题。
- 您创建的租户数量不得超过您的套餐中 `maxWhiteLabeledTenants` 定义的数量。
- 您必须指定 `tenantId` 查询参数，该参数是启用白标的 `parent tenant` 的 id。

我们只需少量参数即可创建一个 `Tenant`：

[inline-code-attrs-start title = '租户创建 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '租户创建 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '租户创建 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** 在失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** 在失败时包含。 **/
    reason?: string
    tenant?: Tenant; // 成功时返回完整的已创建租户。
}
[inline-code-end]