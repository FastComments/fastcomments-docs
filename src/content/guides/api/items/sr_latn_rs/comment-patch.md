[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava ažuriranje jednog komentara.

Notes:

- Ovaj API može ažurirati widget komentara "uživo" ako je potrebno (ovo povećava osnovni `creditsCost` sa `1` na `2`).
  - Ovo može učiniti migraciju komentara između stranica "uživo" (promena `urlId`).
  - Migracije koštaju dodatna `2` kredita jer se stranice prethodno proračunavaju i to je zahtevno za CPU.
- Za razliku od create API-ja, ovaj API NEĆE automatski kreirati korisničke objekte u našem sistemu ako je email naveden.
- Komentari ažurirani putem ovog API-ja i dalje mogu biti provereni na spam ako je potrebno.
- Konfiguracije poput maksimalne dužine komentara, ako su podešene preko Customization Rule admin stranice, primenjivaće se ovde.
- Da biste dozvolili korisnicima da ažuriraju tekst komentara, možete jednostavno navesti `comment` u telu zahteva. Mi ćemo generisati odgovarajući `commentHTML`.
  - Ako definišete i `comment` i `commentHTML`, mi nećemo automatski generisati HTML.
  - Ako korisnik doda pominjanja ili heštegove u svom novom tekstu, oni će i dalje biti obrađeni kao kod `POST` API-ja.
- Prilikom ažuriranja `commenterEmail` na komentaru, najbolje je takođe navesti `userId`. U suprotnom, morate osigurati da korisnik sa tom email adresom pripada vašem tenant-u, inače će zahtev neuspeti.  
- Ako je ciljanim komentar zaključan (`isLocked: true`), zahtev se odbija sa `code: 'locked'`. Prvo otključajte komentar, ažurirajte ga, a zatim ga ponovo zaključajte ako želite.


[inline-code-attrs-start title = 'Minimalni primer cURL PATCH za komentar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
	/** Korisnik koji vrši ažuriranje. Po potrebi se može koristiti da se proveri da li mogu da urede komentar.  **/
    contextUserId?: string
	/** Da li treba da proverimo da li novi komentar izgleda kao spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Da li komentar treba da se pojavi "uživo" korisnicima koji gledaju instance widgeta komentara sa istim urlId. NAPOMENA: Povećava cenu kredita sa 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH odgovora za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
}
[inline-code-end]