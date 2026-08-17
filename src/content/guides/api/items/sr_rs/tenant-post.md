[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава додавање једног `Tenant`.

Креирање `Tenant`-а има следећа ограничења:

- Потребно је `name`.
- Потребно је `domainConfiguration`.
- Следеће вредности не смеју бити наведене приликом креирања `Tenant`-а:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- `signUpDate` не сме бити у будућности.
- `name` не сме бити дужи од `200 characters`.
- `email` не сме бити дужи од `300 characters`.
- `email` мора бити јединствен међу свим tenant-има на FastComments.com.
- Не можете креирати tenant-е ако надређени tenant нема дефинисан валидан `TenantPackage`.
  - Ако је ваш tenant креиран преко FastComments.com, ово не би требало да буде проблем.
- Не можете креирати више tenant-а него што је дефинисано у `maxWhiteLabeledTenants` у вашем пакету.
- Морате навести query параметар `tenantId` који је ID вашег `parent tenant`-а са омогућеним white labeling-ом.

Можемо креирати `Tenant` само са неколико параметара:

[inline-code-attrs-start title = 'Пример cURL захтева за креирање Tenanta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура захтева за креирање Tenanta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за креирање Tenanta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Included on failure. **/
    reason?: string
    tenant?: Tenant; // We return the complete created tenant on success.
}
[inline-code-end]