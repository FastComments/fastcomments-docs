[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at tilføje en enkelt `TenantUser`.

Oprettelse af en `TenantUser` har følgende begrænsninger:

- Et `username` er påkrævet.
- En `email` er påkrævet.
- `signUpDate` må ikke være i fremtiden.
- `locale` skal være på listen over [Understøttede Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` skal være unikt på tværs af hele FastComments.com. Hvis dette er et problem, foreslår vi at bruge SSO i stedet.
- `email` skal være unikt på tværs af hele FastComments.com. Hvis dette er et problem, foreslår vi at bruge SSO i stedet.
- Du kan ikke oprette flere tenant-brugere end defineret under `maxTenantUsers` i din pakke.

Vi kan oprette en `TenantUser` som følger

[inline-code-attrs-start title = 'TenantUser Oprettelse cURL Eksempel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Oprettelse Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Oprettelse Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Included on failure. **/
    reason?: string
    tenantUser?: TenantUser; // We return the complete created tenant user on success.
}
[inline-code-end]
