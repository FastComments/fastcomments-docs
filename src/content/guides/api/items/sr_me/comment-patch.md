[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava ažuriranje jednog komentara.

Napomene:

- Ovaj API može ažurirati widget komentara "uživo" ako se želi (ovo povećava osnovni `creditsCost` sa `1` na `2`).
  - Ovo može omogućiti da migracije komentara između stranica budu "uživo" (promjena `urlId`).
  - Migracije koštaju dodatna `2` kredita jer se stranice prethodno izračunavaju i to je CPU-intenzivno.
- Za razliku od API-ja za kreiranje, ovaj API NEĆE automatski kreirati korisničke objekte u našem sistemu ako je email naveden.
- Komentari ažurirani putem ovog API-ja i dalje mogu biti provjereni na spam ako se želi.
- Konfiguracije poput maksimalne dužine komentara, ako su konfigurirane putem admin stranice Customization Rule, primjenjuju se ovdje.
- Da biste korisnicima omogućili ažuriranje teksta komentara, možete jednostavno navesti `comment` u telu zahtjeva. Mi ćemo generisati rezultirajući `commentHTML`.
  - Ako definišete oba `comment` i `commentHTML`, mi nećemo automatski generisati HTML.
  - Ako korisnik doda pominjanja ili heštagove u svom novom tekstu, oni će se i dalje obraditi kao kod `POST` API-ja.
- Prilikom ažuriranja `commenterEmail` na komentaru, najbolje je takođe navesti `userId`. U suprotnom, morate osigurati da korisnik s tim emailom pripada vašem tenantu, inače će zahtjev biti odbijen.  
- Ako je ciljani komentar zaključan (`isLocked: true`), zahtjev se odbija sa `code: 'locked'`. Prvo otključajte komentar, ažurirajte ga, a zatim ga ponovo zaključajte ako želite.


[inline-code-attrs-start title = 'Minimalni cURL primer za PATCH komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH zahteva za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Korisnik koji vrši ažuriranje. Po želji se može koristiti za proveru da li može urediti komentar.  **/
    contextUserId?: string
	/** Da li treba proveriti da li novi komentar izgleda kao spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Da li komentar treba da se prikazuje "uživo" korisnicima koji gledaju instance widgeta komentara sa istim urlId. NAPOMENA: Udvaja trošak kredita sa 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora PATCH zahteva za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]

---