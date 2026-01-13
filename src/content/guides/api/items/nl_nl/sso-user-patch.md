---
[api-resource-header-start name = 'SSOUser'; route = 'PATCH /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Deze route biedt de mogelijkheid om een enkele SSO-gebruiker bij te werken.

[inline-code-attrs-start title = 'SSOUser bijwerken cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "notfordperfect"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van SSOUser-bijwerkverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Wanneer e-mail of gebruikersnaam wordt gewijzigd, kunt u dit op 'true' zetten om ook de opmerkingen van de gebruiker bij te werken. Dit verdubbelt de kredietkosten. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van SSOUser-bijwerkrespons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchResponse {
    status: 'success' | 'failed'
    /** Bij een mislukking opgenomen. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-does-not-exist'
    /** Bij een mislukking opgenomen. **/
    reason?: string
    user?: SSOUser; // We geven de volledig bijgewerkte gebruiker terug bij succes.
}
[inline-code-end]


---