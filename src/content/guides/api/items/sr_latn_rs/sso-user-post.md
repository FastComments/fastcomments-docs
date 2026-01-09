[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava kreiranje jednog SSO korisnika.

Pokušaj kreiranja dva korisnika sa istim ID-jem će rezultovati greškom.

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje SSOUser-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

U ovom primeru navodimo `groupIds` za kontrolu pristupa, ali ovo je opciono.

[inline-code-attrs-start title = 'Struktura zahteva za kreiranje SSOUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za kreiranje SSOUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Uključeno pri neuspehu. **/
    reason?: string
    user?: SSOUser; // Vraćamo kreiranog korisnika pri uspehu.
}
[inline-code-end]

#### Napomena o integraciji

Podaci poslati preko API-ja mogu se jednostavno prebrisati prosleđivanjem drugačijeg SSO User HMAC payload-a. Na primer, ako postavite username putem API-ja, ali zatim prosledite drugačiji putem SSO toka pri učitavanju stranice, automatski ćemo ažurirati njihov username.

Nećemo ažurirati parametre korisnika u ovom toku osim ako ih eksplicitno ne navedete ili postavite na null (ne undefined).