[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućava kreiranje jednog SSO korisnika.

Pokušaj kreiranja dva korisnika sa istim ID-jem rezultiraće greškom.

[inline-code-attrs-start title = 'SSOUser kreiranje — primjer cURL zahtjeva'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

U ovom primjeru navodimo `groupIds` za kontrolu pristupa, ali ovo je opciono.

[inline-code-attrs-start title = 'Struktura zahtjeva za kreiranje SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za kreiranje SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    user?: SSOUser; // U slučaju uspjeha vraćamo kreiranog korisnika.
}
[inline-code-end]

#### Napomena za integraciju

Podaci poslati putem API-ja mogu biti prebrisani jednostavnim slanjem drugačijeg SSO User HMAC payload. Na primjer, ako
postavite username putem API-ja, ali zatim pošaljete drugačiji putem SSO toka pri učitavanju stranice, mi ćemo automatski ažurirati
njihov username.

Nećemo ažurirati parametre korisnika u ovom toku osim ako ih izričito ne navedete ili ih ne postavite na null (ne undefined).