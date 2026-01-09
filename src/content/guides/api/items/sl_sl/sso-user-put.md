[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ta končna točka omogoča posodobitev enega SSO uporabnika.

[inline-code-attrs-start title = 'Primer cURL za posodobitev SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

V tem primeru navedemo `groupIds` za nadzor dostopa, vendar je to neobvezno.

[inline-code-attrs-start title = 'Struktura zahtevka za posodobitev SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Ko se spremeni e-pošta ali uporabniško ime, lahko to nastavite na true, da posodobite tudi komentarje uporabnika. To bo podvojilo porabo kreditov. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za posodobitev SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Vključeno ob napaki. **/
    reason?: string
    user?: SSOUser; // Na uspeh vrnemo posodobljenega uporabnika.
}
[inline-code-end]


---