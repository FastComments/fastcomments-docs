[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Ця кінцева точка API надає можливість оновити `Tenant` за `id`.

Оновлення `Tenant` має такі обмеження:

- Наступні значення не можуть бути оновлені:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- Значення `signUpDate` не може бути в майбутньому.
- Поле `name` не може бути довшим за `200 characters`.
- Поле `email` не може бути довшим за `300 characters`.
- Значення `email` має бути унікальним серед усіх тенантів FastComments.com.
- Якщо встановити `billingInfoValid` в `true`, то `billingInfo` має бути надано в тому самому запиті.
- Ви не можете оновити `packageId`, пов'язаний з вашим власним тенантом.
- Ви не можете оновити `paymentFrequency`, пов'язаний з вашим власним тенантом.

[inline-code-attrs-start title = 'Приклад cURL-запиту PATCH для Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту Tenant PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді Tenant PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Включено у випадку невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Включено у випадку невдачі. **/
    reason?: string
}
[inline-code-end]