[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućuje ažuriranje jednog komentara.

Napomene:

- Ovaj API može ažurirati widget komentara "uživo" ako želite (to povećava osnovni `creditsCost` s `1` na `2`).
  - To može učiniti migraciju komentara između stranica "uživo" (promjenom `urlId`).
  - Migracije koštaju dodatna `2` kredita jer se stranice prethodno izračunavaju i to je CPU-intenzivno.
- Za razliku od create API-ja, ovaj API NEĆE automatski kreirati korisničke objekte u našem sustavu ako je naveden email.
- Komentari ažurirani putem ovog API-ja i dalje se mogu provjeriti na spam ako želite.
- Konfiguracije poput maksimalne duljine komentara, ako su postavljene putem stranice administracije Customization Rule, primjenjuju se ovdje.
- Da biste omogućili korisnicima ažuriranje teksta komentara, možete jednostavno navesti `comment` u tijelu zahtjeva. Mi ćemo generirati rezultirajući `commentHTML`.
  - Ako definirate oba `comment` i `commentHTML`, nećemo automatski generirati HTML.
  - Ako korisnik u svom novom tekstu doda spominjanja ili hashtagove, oni će se i dalje obraditi kao kod `POST` API-ja.
- Pri ažuriranju `commenterEmail` na komentaru, najbolje je također navesti `userId`. Inače morate osigurati da korisnik s tim emailom pripada vašem tenantu, inače će zahtjev biti odbijen.  


[inline-code-attrs-start title = 'Minimalni primjer cURL PATCH zahtjeva za komentar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH zahtjeva za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Korisnik koji vrši ažuriranje. Po želji se može koristiti za provjeru mogu li urediti komentar.  **/
    contextUserId?: string
	/** Trebamo li provjeriti izgleda li novi komentar kao spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Treba li komentar biti prikazan "uživo" korisnicima koji pregledavaju instance widgeta komentara s istim urlId. NAPOMENA: udvostručuje trošak kredita s 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora PATCH zahtjeva za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Uključeno pri neuspjehu. **/
    reason?: string
}
[inline-code-end]