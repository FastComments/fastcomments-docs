[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Deze route biedt de mogelijkheid om een enkele SSO-gebruiker bij te werken.

[inline-code-attrs-start title = 'SSOUser Update cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

In dit voorbeeld geven we `groupIds` op voor toegangscontrole, maar dit is optioneel.

[inline-code-attrs-start title = 'SSOUser Update Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Wanneer e-mail of gebruikersnaam wordt gewijzigd, kunt u dit op 'true' zetten om ook de opmerkingen van de gebruiker bij te werken. Dit verdubbelt de kredietkosten. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Update Antwoordstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Opgenomen bij mislukking. **/
    reason?: string
    user?: SSOUser; // We geven de bijgewerkte gebruiker terug bij succes.
}
[inline-code-end]