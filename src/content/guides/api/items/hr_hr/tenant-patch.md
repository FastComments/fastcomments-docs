[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Ova krajnja točka API-ja omogućuje ažuriranje `Tenant` prema `id`.

Ažuriranje `Tenant` ima sljedeća ograničenja:

- Sljedeće vrijednosti se ne smiju ažurirati:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
  - `managedByTenantId`
- `signUpDate` ne smije biti u budućnosti.
- `name` ne smije biti duže od `200 characters`.
- `email` ne smije biti duže od `300 characters`.
- `email` mora biti jedinstven među svim tenantima FastComments.com.
- Kada se `billingInfoValid` postavi na `true`, `billingInfo` mora biti dostavljen u istom zahtjevu.
- Ne smijete ažurirati `packageId` povezan s vašim vlastitim tenantom.
- Ne smijete ažurirati `paymentFrequency` povezan s vašim vlastitim tenantom.

[inline-code-attrs-start title = 'Primjer cURL PATCH za Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH zahtjeva za Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'; 
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]

---