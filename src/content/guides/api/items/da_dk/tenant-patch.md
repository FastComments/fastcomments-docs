[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Dette API-endpoint giver mulighed for at opdatere en `Tenant` efter `id`.

Opdatering af en `Tenant` har følgende begrænsninger:

- Følgende værdier kan ikke opdateres:
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` må ikke være i fremtiden.
- `name` må ikke være længere end `200 tegn`.
- `email` må ikke være længere end `300 tegn`.
- `email` skal være unik på tværs af alle FastComments.com tenants.
- Når du sætter `billingInfoValid` til `true`, skal `billingInfo` angives i den samme anmodning.
- Du kan ikke opdatere `packageId` tilknyttet din egen tenant.
- Du kan ikke opdatere `paymentFrequency` tilknyttet din egen tenant.

[inline-code-attrs-start title = 'Tenant PATCH cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant PATCH Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
