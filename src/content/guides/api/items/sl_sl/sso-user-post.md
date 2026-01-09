[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ta končna točka omogoča ustvarjanje enega SSO uporabnika.

Poskus ustvarjanja dveh uporabnikov z istim ID bo povzročil napako.

[inline-code-attrs-start title = 'Primer cURL zahteve za ustvarjanje SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

V tem primeru določimo `groupIds` za nadzor dostopa, vendar je to izbirno.

[inline-code-attrs-start title = 'Struktura zahteve za ustvarjanje SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ustvarjanje SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Vključeno ob napaki. **/
    reason?: string
    user?: SSOUser; // Na uspeh vrnemo ustvarjenega uporabnika.
}
[inline-code-end]

#### Opomba o integraciji

Podatke, poslane preko API-ja, je mogoče preglasiti preprosto tako, da posredujete drugačen SSO User HMAC payload. Na primer, če
nastavite uporabniško ime prek API-ja, a nato ob nalaganju strani skozi SSO tok posredujete drugačno, bomo samodejno posodobili
njihovo uporabniško ime.

V tem toku ne bomo posodabljali uporabniških parametrov, razen če jih izrecno določite ali nastavite na null (ne undefined).