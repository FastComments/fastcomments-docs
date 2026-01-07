[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at oprette en enkelt SSO-bruger.

Forsøg på at oprette to brugere med det samme ID vil resultere i en fejl.

[inline-code-attrs-start title = 'SSOUser Oprettelse cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

I dette eksempel angiver vi `groupIds` til adgangskontrol, men dette er valgfrit.

[inline-code-attrs-start title = 'SSOUser Oprettelse Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Oprettelse Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the created user on success.
}
[inline-code-end]

#### Integrationsbemærkning

Data sendt af API'et kan tilsidesættes simpelthen ved at sende en anden SSO User HMAC payload. For eksempel, hvis
du sætter et brugernavn via API'et, men derefter sender et andet via SSO-flowet ved sideindlæsning, vil vi automatisk opdatere
deres brugernavn.

Vi vil ikke opdatere brugerparametre i dette flow, medmindre du eksplicit angiver dem eller sætter dem til null (ikke undefined).
