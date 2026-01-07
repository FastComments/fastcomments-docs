[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at erstatte en enkelt `TenantUser`.

Erstatning af en `TenantUser` har følgende begrænsninger:

- `signUpDate` må ikke være i fremtiden.
- `locale` skal være på listen over [Understøttede Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` skal være unikt på tværs af hele FastComments.com. Hvis dette er et problem, foreslår vi at bruge SSO i stedet.
- `email` skal være unikt på tværs af hele FastComments.com. Hvis dette er et problem, foreslår vi at bruge SSO i stedet.
- Du kan ikke opdatere `tenantId` for en bruger.

Vi kan oprette en `TenantUser` som følger

[inline-code-attrs-start title = 'TenantUser Erstatning cURL Eksempel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Erstatning Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** When email or username is changed, you can set this to true to also update the user's comments. This will double the credit cost. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Erstatning Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
