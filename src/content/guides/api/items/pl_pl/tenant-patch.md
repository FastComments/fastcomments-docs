[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Ten punkt końcowy API umożliwia aktualizację `Tenant` według `id`.

Aktualizacja `Tenant` ma następujące ograniczenia:

- Następujące wartości nie mogą być aktualizowane:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` nie może być w przyszłości.
- `name` nie może być dłuższe niż `200 characters`.
- `email` nie może być dłuższe niż `300 characters`.
- `email` musi być unikalny wśród wszystkich tenantów FastComments.com.
- Jeśli ustawiasz `billingInfoValid` na `true`, `billingInfo` musi być podane w tym samym żądaniu.
- Nie możesz zaktualizować `packageId` powiązanego z Twoim tenantem.
- Nie możesz zaktualizować `paymentFrequency` powiązanego z Twoim tenantem.

[inline-code-attrs-start title = 'Przykład żądania cURL PATCH dla Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania PATCH dla Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi PATCH dla Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]