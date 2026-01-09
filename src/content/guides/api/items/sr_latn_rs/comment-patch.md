[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava ažuriranje pojedinačnog komentara.

Napomene:

- Ovaj API može ažurirati comment widget "live" ako je željeno (ovo povećava base `creditsCost` from `1` to `2`).
  - Ovo može omogućiti migraciju komentara između stranica "live" (promena `urlId`).
  - Migracije koštaju dodatna `2` kredita jer se stranice prethodno izračunavaju i ovo je CPU intenzivno.
- Za razliku od create API-ja, ovaj API NEĆE automatski kreirati objekte korisnika u našem sistemu ako je email naveden.
- Komentari ažurirani putem ovog API-ja i dalje se mogu proveravati na spam ako je potrebno.
- Konfiguracije kao što su maksimalna dužina komentara, ako su postavljene preko Customization Rule admin stranice, biće primenjene ovde.
- Da biste omogućili korisnicima da ažuriraju tekst svog komentara, možete samo navesti `comment` u telu zahteva. Mi ćemo generisati rezultujući `commentHTML`.
  - Ako definišete i `comment` i `commentHTML`, mi nećemo automatski generisati HTML.
  - Ako korisnik doda mentions ili hashtags u svom novom tekstu, to će i dalje biti obrađeno kao kod `POST` API-ja.
- Kada ažurirate `commenterEmail` na komentaru, najbolje je da takođe navedete `userId`. Inače, morate osigurati da korisnik sa tim emailom pripada vašem tenant-u, inače će zahtev propasti.  


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
	/** Korisnik koji vrši ažuriranje. Po želji može biti korišćen za proveru da li mogu da uređuju komentar.  **/
    contextUserId?: string
	/** Da li da proverimo da li novi komentar izgleda kao spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Da li komentar treba da se pojavi "live" korisnicima koji gledaju instance widgeta za komentare sa istim urlId. NAPOMENA: Udvostručuje trošak kredita sa 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH odgovora za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Uključeno u slučaju greške. **/
    reason?: string
}
[inline-code-end]

---