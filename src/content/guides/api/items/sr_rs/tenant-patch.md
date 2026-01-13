[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Овај API крајња тачка омогућава ажурирање `Tenant` по `id`.

Ажурирање `Tenant` има следећа ограничења:

- Следеће вредности не могу бити ажуриране:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` не може бити у будућности.
- `name` не може бити дужи од `200 characters`.
- `email` не може бити дужи од `300 characters`.
- `email` мора бити јединствен међу свим FastComments.com tenant-има.
- Када се постави `billingInfoValid` на `true`, `billingInfo` мора бити послат у истом захтеву.
- Не можете ажурирати `packageId` повезан са вашим сопственим tenant-ом.
- Не можете ажурирати `paymentFrequency` повезан са вашим сопственим tenant-ом.

[inline-code-attrs-start title = 'Пример cURL захтева за Tenant PATCH'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за Tenant PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за Tenant PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]