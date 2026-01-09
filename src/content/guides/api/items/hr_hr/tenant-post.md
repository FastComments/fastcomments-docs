[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje dodavanje jednog `Tenant`.

Kreiranje `Tenant`-a ima sljedeća ograničenja:

- `name` je obavezno.
- `domainConfiguration` je obavezno.
- Sljedeće vrijednosti se ne smiju navesti prilikom kreiranja `Tenant`-a:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- `signUpDate` ne smije biti u budućnosti.
- `name` ne smije biti duži od `200 characters`.
- `email` ne smije biti duži od `300 characters`.
- `email` mora biti jedinstven među svim tenantima FastComments.com.
- Ne smijete kreirati tenante ako roditeljski tenant nema definirani valjani `TenantPackage`.
  - Ako je vaš tenant kreiran putem FastComments.com, to ne bi trebao biti problem.
- Ne smijete kreirati više tenant-a nego što je definirano u `maxWhiteLabeledTenants` u vašem paketu.
- Morate navesti query parametar `tenantId` koji je id vašeg `parent tenant`-a s omogućenim white labelingom.

Možemo kreirati `Tenant` koristeći samo nekoliko parametara:

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za kreiranje Tenant-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahtjeva za kreiranje Tenant-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    tenant?: Tenant; // U slučaju uspjeha vraćamo kompletan stvoreni tenant.
}
[inline-code-end]