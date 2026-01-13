[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Ovaj ruta omogućava dodavanje jednog `Tenant`.

Kreiranje `Tenant` ima sledeća ograničenja:

- `name` je obavezno.
- `domainConfiguration` je obavezno.
- Sledeće vrednosti ne smeju biti prosleđene prilikom kreiranja `Tenant`:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- `signUpDate` ne sme da bude u budućnosti.
- `name` ne sme biti duže od `200 characters`.
- `email` ne sme biti duži od `300 characters`.
- `email` mora biti jedinstven za sve FastComments.com tenante.
- Ne možete kreirati tenante ako roditeljski tenant nema definisan validan `TenantPackage`.
  - Ako je vaš tenant kreiran putem FastComments.com, ovo ne bi trebalo da predstavlja problem.
- Ne možete kreirati više tenant-a nego što je definisano u `maxWhiteLabeledTenants` u vašem paketu.
- Morate navesti query parametar `tenantId` koji je id vašeg `parent tenant` sa uključenim white labeling-om.

Možemo kreirati `Tenant` sa samo nekoliko parametara:

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje Tenant-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahteva za kreiranje Tenant-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za kreiranje Tenant-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    tenant?: Tenant; // Vraćamo kompletan kreirani tenant pri uspehu.
}
[inline-code-end]