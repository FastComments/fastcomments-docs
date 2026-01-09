[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje ažuriranje jednog SSO korisnika.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za ažuriranje SSO korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

U ovom primjeru navodimo `groupIds` za kontrolu pristupa, ali to je neobavezno.

[inline-code-attrs-start title = 'Struktura zahtjeva za ažuriranje SSO korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Kada se promijeni email ili korisničko ime, možete postaviti ovo na true da biste također ažurirali komentare korisnika. To će udvostručiti trošak kredita. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ažuriranje SSO korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    user?: SSOUser; // Na uspjehu vraćamo ažuriranog korisnika.
}
[inline-code-end]


---