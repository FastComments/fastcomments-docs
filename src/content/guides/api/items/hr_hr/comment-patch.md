[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućuje ažuriranje pojedinačnog komentara.

Napomene:

- Ovaj API može ažurirati widget komentara "uživo" ako se želi (ovo povećava osnovni `creditsCost` s `1` na `2`).
  - Ovo može učiniti migracije komentara između stranica "uživo" (promjena `urlId`).
  - Migracije koštaju dodatna `2` kredita jer se stranice prethodno izračunavaju i to je CPU-intenzivno.
- Za razliku od API-ja za kreiranje, ovaj API NEće automatski stvoriti korisničke objekte u našem sustavu ako je e-mail naveden.
- Komentari ažurirani putem ovog API-ja i dalje se mogu provjeriti na spam ako je potrebno.
- Konfiguracije poput maksimalne duljine komentara, ako su podešene putem administratorske stranice pravila prilagodbe, primjenjivat će se ovdje.
- Da biste korisnicima omogućili ažuriranje teksta komentara, možete jednostavno navesti `comment` u tijelu zahtjeva. Mi ćemo generirati odgovarajući `commentHTML`.
  - Ako definirate i `comment` i `commentHTML`, nećemo automatski generirati HTML.
  - Ako korisnik doda spominjanja ili hashtagove u svom novom tekstu, oni će i dalje biti obrađeni poput `POST` API-ja.
- Prilikom ažuriranja `commenterEmail` na komentaru, najbolje je također navesti `userId`. Inače morate osigurati da korisnik s tom e-mail adresom pripada vašem tenantu, inače će zahtjev propasti.  
- Ako je ciljani komentar zaključan (`isLocked: true`), zahtjev se odbija s `code: 'locked'`. Prvo otključajte komentar, ažurirajte ga, a zatim ga ponovno zaključajte ako želite.


[inline-code-attrs-start title = 'Minimalni primjer PATCH zahtjeva za komentar (cURL)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
	/** Korisnik koji obavlja ažuriranje. Po želji se može koristiti za provjeru mogu li uređivati komentar.  **/
    contextUserId?: string
	/** Trebamo li provjeriti izgleda li novi komentar kao spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Treba li komentar biti prikazan "uživo" korisnicima koji gledaju instance widgeta komentara s istim urlId-om. NAPOMENA: Udvostručuje trošak kredita s 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora PATCH zahtjeva za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju pogreške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Uključeno u slučaju pogreške. **/
    reason?: string
}
[inline-code-end]

---