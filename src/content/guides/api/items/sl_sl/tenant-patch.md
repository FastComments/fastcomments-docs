[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Ta API endpoint omogoča posodobitev `Tenant` po `id`.

Pri posodabljanju `Tenant` veljajo naslednje omejitve:

- Naslednje vrednosti ni mogoče posodobiti:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` ne sme biti v prihodnosti.
- `name` ne sme presegati dolžine `200 characters`.
- `email` ne sme presegati dolžine `300 characters`.
- `email` mora biti unikaten za vse FastComments.com tenants.
- Ko nastavite `billingInfoValid` na `true`, mora biti `billingInfo` priložen v istem zahtevku.
- Ne smete posodobiti `packageId`, povezanega z vašim tenantom.
- Ne smete posodobiti `paymentFrequency`, povezanega z vašim tenantom.

[inline-code-attrs-start title = 'Primer cURL PATCH zahteve za Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH zahtevka za Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH odgovora za Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Vključeno ob napaki. **/
    reason?: string
}
[inline-code-end]