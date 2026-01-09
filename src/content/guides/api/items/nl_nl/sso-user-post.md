[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Deze route maakt het mogelijk één SSO-gebruiker aan te maken.

Als u probeert twee gebruikers met hetzelfde ID aan te maken, resulteert dat in een fout.

[inline-code-attrs-start title = 'SSOUser aanmaken cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

In dit voorbeeld specificeren we `groupIds` voor toegangscontrole, maar dit is optioneel.

[inline-code-attrs-start title = 'SSOUser aanmaken - structuur van de aanvraag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser aanmaken - structuur van het antwoord'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Opgenomen bij mislukking. **/
    reason?: string
    user?: SSOUser; // We geven de aangemaakte gebruiker terug bij succes.
}
[inline-code-end]

#### Integratie-opmerking

Door de API doorgegeven gegevens kunnen worden overschreven door simpelweg een andere SSO User HMAC-payload mee te geven. Als u bijvoorbeeld een gebruikersnaam instelt via de API, maar bij het SSO-proces bij het laden van de pagina een andere opgeeft, zullen wij hun gebruikersnaam automatisch bijwerken.

We zullen gebruikersparameters in deze flow niet bijwerken tenzij u ze expliciet opgeeft of ze op null zet (niet undefined).