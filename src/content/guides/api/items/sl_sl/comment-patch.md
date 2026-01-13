[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ta API endpoint omogoča posodobitev enega komentarja.

Notes:

- Ta API lahko po želji posodobi pripomoček za komentarje "live" (to poveča osnovni `creditsCost` z `1` na `2`).
  - To omogoča, da je migracija komentarjev med stranmi "v živo" (spreminjanje `urlId`).
  - Migracije stanejo dodatne `2` kredite, saj se strani predizračunajo in je to intenzivno za CPU.
- V nasprotju z API-jem za ustvarjanje, ta API NE bo samodejno ustvaril uporabniških objektov v našem sistemu, če je naveden e-poštni naslov.
- Komentarje posodobljene preko tega API-ja je mogoče po želji še vedno preveriti na neželeno pošto (spam).
- Konfiguracije, kot je največja dolžina komentarja, če so nastavljene prek strani za upravljanje pravil prilagoditve (Customization Rule), bodo veljale tudi tu.
- Da omogočite uporabnikom, da posodobijo besedilo komentarja, lahko v telesu zahteve preprosto navedete `comment`. Mi bomo ustvarili ustrezen `commentHTML`.
  - Če določite oba `comment` in `commentHTML`, HTML ne bomo samodejno ustvarili.
  - Če uporabnik v novo besedilo doda omembe ali oznake, bo to še vedno obdelano enako kot pri `POST` API-ju.
- Ko posodabljate `commenterEmail` pri komentarju, je najbolje, da prav tako navedete `userId`. V nasprotnem primeru morate zagotoviti, da uporabnik s tem e-poštnim naslovom pripada vašemu tenant, sicer bo zahteva neuspešna.  


[inline-code-attrs-start title = 'Minimalen primer PATCH cURL zahteve za komentar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za PATCH komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Uporabnik, ki izvaja posodobitev. Po želji se lahko uporabi za preverjanje, ali lahko ureja komentar.  **/
    contextUserId?: string
	/** Ali naj preverimo, ali nov komentar izgleda kot spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Ali naj se komentar prikaže "v živo" uporabnikom, ki gledajo instance pripomočka za komentarje z istim urlId. OPOMBA: Podvoji strošek kreditov s 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora PATCH komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Vključeno ob napaki. **/
    reason?: string
}
[inline-code-end]