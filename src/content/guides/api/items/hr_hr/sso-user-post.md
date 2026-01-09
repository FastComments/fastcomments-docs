[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje kreiranje jednog SSO korisnika.

Pokušaj kreiranja dvaju korisnika s istim ID-om rezultirat će pogreškom.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za kreiranje SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

U ovom primjeru navodimo `groupIds` za kontrolu pristupa, ali to je opcionalno.

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
    user?: SSOUser; // Vraćamo kreiranog korisnika u slučaju uspjeha.
}
[inline-code-end]

#### Napomena za integraciju

Podaci poslani putem API-ja mogu se prebrisati jednostavnim slanjem drugačijeg SSO User HMAC payloada. Na primjer, ako postavite korisničko ime putem API-ja, ali zatim proslijedite drugačije korisničko ime kroz SSO tok prilikom učitavanja stranice, automatski ćemo ažurirati njihovo korisničko ime.

Nećemo ažurirati parametre korisnika u ovom toku osim ako ih eksplicitno ne navedete ili ne postavite na null (ne undefined).

---