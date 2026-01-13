[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Deze route biedt de mogelijkheid om een enkele `Tenant` toe te voegen.

Het aanmaken van een `Tenant` kent de volgende beperkingen:

- Een `name` is verplicht.
- `domainConfiguration` is verplicht.
- De volgende waarden mogen niet worden meegegeven bij het aanmaken van een `Tenant`:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- De `signUpDate` mag niet in de toekomst liggen.
- De `name` mag niet langer zijn dan `200 characters`.
- De `email` mag niet langer zijn dan `300 characters`.
- De `email` moet uniek zijn over alle FastComments.com tenants.
- U mag geen tenants aanmaken als de parent tenant geen geldige `TenantPackage` heeft gedefinieerd.
  - Als uw tenant via FastComments.com is aangemaakt, zou dit geen probleem moeten zijn.
- U mag niet meer tenants aanmaken dan gedefinieerd onder `maxWhiteLabeledTenants` in uw package.
- U moet de queryparameter `tenantId` opgeven, dit is de id van uw `parent tenant` met white labeling ingeschakeld.

We kunnen een `Tenant` aanmaken met slechts een paar parameters:

[inline-code-attrs-start title = 'Voorbeeld cURL-aanvraag voor Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structuur van Tenant-aanmaakverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Tenant-aanmaakrespons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Inbegrepen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Inbegrepen bij mislukking. **/
    reason?: string
    tenant?: Tenant; // We geven de volledig aangemaakte tenant terug bij succes.
}
[inline-code-end]