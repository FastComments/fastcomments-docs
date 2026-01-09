[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava dodavanje jednog `Tenant`.

Kreiranje `Tenant` ima sljedeća ograničenja:

- Polje `name` je obavezno.
- Polje `domainConfiguration` je obavezno.
- Sljedeće vrijednosti se ne smiju navesti pri kreiranju `Tenant`:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- Vrijednost `signUpDate` ne smije biti u budućnosti.
- Polje `name` ne smije biti duže od `200 characters`.
- Polje `email` ne smije biti duže od `300 characters`.
- Vrijednost `email` mora biti jedinstvena među svim tenantima na FastComments.com.
- Ne smijete kreirati tenant-e ako roditeljski tenant nema definisan važeći `TenantPackage`.
  - Ako je vaš tenant kreiran putem FastComments.com, ovo ne bi trebao biti problem.
- Ne smijete kreirati više tenant-a nego što je definirano u `maxWhiteLabeledTenants` u vašem paketu.
- Morate navesti query parametar `tenantId` koji je id vašeg `parent tenant` sa uključenim white labeling-om.

Možemo kreirati `Tenant` sa samo nekoliko parametara:

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za kreiranje Tenanta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahtjeva za kreiranje Tenanta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za kreiranje Tenanta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    tenant?: Tenant; // Vraćamo kompletan kreirani tenant u slučaju uspjeha.
}
[inline-code-end]

---