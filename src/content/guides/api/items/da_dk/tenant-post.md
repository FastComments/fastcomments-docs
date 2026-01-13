[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at tilføje en enkelt `Tenant`.

Oprettelse af en `Tenant` har følgende begrænsninger:

- Et `name` er påkrævet.
- `domainConfiguration` er påkrævet.
- Følgende værdier må ikke angives ved oprettelse af en `Tenant`:
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
- `signUpDate` må ikke være i fremtiden.
- `name` må ikke være længere end `200 tegn`.
- `email` må ikke være længere end `300 tegn`.
- `email` skal være unik på tværs af alle FastComments.com tenants.
- Du kan ikke oprette tenants, hvis den overordnede tenant ikke har en gyldig `TenantPackage` defineret.
  - Hvis din tenant blev oprettet via FastComments.com, bør dette ikke være et problem.
- Du kan ikke oprette flere tenants end defineret under `maxWhiteLabeledTenants` i din pakke.
- Du skal angive `tenantId`-forespørgselsparameteren, som er id'et for din `overordnede tenant` med white labeling aktiveret.

Vi kan oprette en `Tenant` med kun få parametre:

[inline-code-attrs-start title = 'Tenant Oprettelse cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Tenant Oprettelse Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Oprettelse Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Included on failure. **/
    reason?: string
    tenant?: Tenant; // We return the complete created tenant on success.
}
[inline-code-end]
