[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Denne rute giver mulighed for at sende et login-link til en enkelt `TenantUser`.

Nyttigt ved batch-oprettelse af brugere uden at skulle instruere dem i, hvordan man logger ind på FastComments.com. Dette sender dem bare et "magic link" til login, der
udløber efter `30 dage`.

Følgende begrænsninger eksisterer for at sende et login-link til en `TenantUser`:
- `TenantUser` skal allerede eksistere.
- Du skal have adgang til at administrere den `Tenant`, som `TenantUser` tilhører.

Vi kan sende et login-link til en `TenantUser` som følger:

[inline-code-attrs-start title = 'TenantUser Login-link cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Dette sender en e-mail som `Bob hos TenantName inviterer dig til at være moderator...`

[inline-code-attrs-start title = 'TenantUser Login-link Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Login-link Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
