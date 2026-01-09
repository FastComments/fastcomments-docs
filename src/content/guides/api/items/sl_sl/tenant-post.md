[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Ta ruta omogoča dodajanje enega `Tenant`.

Ustvarjanje `Tenant` ima naslednje omejitve:

- Polje `name` je obvezno.
- `domainConfiguration` je obvezno.
- Naslednjih vrednosti ni dovoljeno posredovati pri ustvarjanju `Tenant`:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- Datum `signUpDate` ne sme biti v prihodnosti.
- Ime `name` ne sme biti daljše od `200 characters`.
- Elektronski naslov `email` ne sme biti daljši od `300 characters`.
- Elektronski naslov `email` mora biti unikaten med vsemi najemniki na FastComments.com.
- Ne smete ustvarjati najemnikov, če nadrejeni najemnik nima veljavnega `TenantPackage`.
  - Če je vaš najemnik ustvarjen preko FastComments.com, to ne bi smelo povzročati težav.
- Ne smete ustvariti več najemnikov, kot je določeno v `maxWhiteLabeledTenants` v vašem paketu.
- Obvezno morate določiti poizvedbeni parameter `tenantId`, ki je id vašega `parent tenant` z omogočenim white labelingom.

Za ustvarjanje `Tenant` potrebujemo le nekaj parametrov:

[inline-code-attrs-start title = 'Primer cURL zahteve za ustvarjanje najemnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahteve za ustvarjanje najemnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ustvarjanje najemnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Vključeno ob napaki. **/
    reason?: string
    tenant?: Tenant; // Ob uspehu vrnemo popolnoma ustvarjenega najemnika.
}
[inline-code-end]