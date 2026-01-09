[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Ова API крајња тачка омогућава ажурирање `Tenant` по `id`.

Ажурирање `Tenant` има следећа ограничења:

- Следеће вредности се не могу ажурирати:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` не смије бити у будућности.
- `name` не може бити дуже од `200 characters`.
- `email` не може бити дуже од `300 characters`.
- `email` мора бити јединствен међу свим FastComments.com tenant-има.
- Када се `billingInfoValid` постави на `true`, `billingInfo` мора бити укључен у истом захтјеву.
- Не можете ажурирати `packageId` повезан са вашим сопственим tenant-ом.
- Не можете ажурирати `paymentFrequency` повезан са вашим сопственим tenant-ом.

[inline-code-attrs-start title = 'Tenant PATCH cURL Пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH Структура захтјева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH Структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Укључено при неуспјеху. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Укључено при неуспјеху. **/
    reason?: string
}
[inline-code-end]

---