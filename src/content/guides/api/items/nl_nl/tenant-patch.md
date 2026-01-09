[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Dit API-eindpunt maakt het mogelijk een `Tenant` op basis van `id` bij te werken.

Het bijwerken van een `Tenant` heeft de volgende beperkingen:

- De volgende waarden mogen niet worden bijgewerkt:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- De `signUpDate` mag niet in de toekomst liggen.
- De `name` mag niet langer zijn dan `200 characters`.
- De `email` mag niet langer zijn dan `300 characters`.
- De `email` moet uniek zijn voor alle tenants van FastComments.com.
- Wanneer `billingInfoValid` op `true` wordt gezet, moet `billingInfo` in hetzelfde verzoek worden meegegeven.
- U mag de `packageId` die aan uw eigen tenant is gekoppeld niet bijwerken.
- U mag de `paymentFrequency` die aan uw eigen tenant is gekoppeld niet bijwerken.

[inline-code-attrs-start title = 'Tenant PATCH cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH Aanvraagstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH Antwoordstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]

---