[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Ova API krajnja tačka omogućava ažuriranje `Tenant` po `id`.

Ažuriranje `Tenant` ima sledeća ograničenja:

- Sledeće vrednosti se ne mogu ažurirati:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- Vrednost `signUpDate` ne sme biti u budućnosti.
- Vrednost `name` ne sme biti duža od `200 characters`.
- Vrednost `email` ne sme biti duža od `300 characters`.
- Vrednost `email` mora biti jedinstvena među svim tenant-ima FastComments.com.
- Kada se `billingInfoValid` postavi na `true`, `billingInfo` mora biti dostavljen u istom zahtevu.
- Ne možete da ažurirate `packageId` koji je povezan sa vašim tenant-om.
- Ne možete da ažurirate `paymentFrequency` koji je povezan sa vašim tenant-om.

[inline-code-attrs-start title = 'Primer cURL PATCH zahteva za Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH zahteva za Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Tenant PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Uključeno u slučaju greške. **/
    reason?: string
}
[inline-code-end]