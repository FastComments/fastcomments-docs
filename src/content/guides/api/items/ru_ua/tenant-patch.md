[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность обновить `Tenant` по `id`.

Обновление `Tenant` имеет следующие ограничения:

- Следующие значения нельзя обновлять:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` не может быть в будущем.
- `name` не может быть длиннее `200 characters`.
- `email` не может быть длиннее `300 characters`.
- `email` должен быть уникальным среди всех тенантов FastComments.com.
- При установке `billingInfoValid` в `true`, `billingInfo` должен быть предоставлен в том же запросе.
- Вы не можете обновлять `packageId`, связанный с вашим собственным тенантом.
- Вы не можете обновлять `paymentFrequency`, связанный с вашим собственным тенантом.

[inline-code-attrs-start title = 'Пример cURL-запроса для Tenant PATCH'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса Tenant PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа Tenant PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Указывается при неудаче. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Указывается при неудаче. **/
    reason?: string
}
[inline-code-end]