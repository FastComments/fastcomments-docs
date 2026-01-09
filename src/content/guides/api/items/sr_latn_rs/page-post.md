[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava kreiranje stranica.

Uobičajen slučaj upotrebe je kontrola pristupa.

Napomene:

- Ako ste komentarisali u niti komentara, ili pozvali API da kreirate `Comment`, već ste kreirali `Page` objekat! Možete ga pokušati dohvatiti putem `/by-url-id` `Page` rute, prosleđujući isti `urlId` koji je prosleđen comment widgetu.
- Struktura `Page` sadrži neke **izračunate** vrednosti.
  Trenutno su to `commentCount` i `rootCommentCount`.
  Popunjavaju se automatski i ne mogu se postaviti preko API-ja. Pokušaj da se postave prouzrokovaće da API vrati grešku.

[inline-code-attrs-start title = 'Primer cURL zahteva za Page POST'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura Page POST zahteva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Page POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    /** Kreirana stranica. **/
    page?: Page
}
[inline-code-end]

---