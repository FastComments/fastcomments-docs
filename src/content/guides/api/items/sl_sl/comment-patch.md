[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ta API vmesnik omogoča posodobitev enega komentarja.

Notes:

- Ta API lahko po želji posodobi widget za komentarje "v živo" (to poveča osnovni `creditsCost` s `1` na `2`).
  - To lahko omogoči, da so migracije komentarjev med stranmi "v živo" (spreminjanje `urlId`).
  - Migracije stanejo dodatne `2` kredite, saj so strani predizračunane in je to CPU-intenzivno.
- V nasprotju z API-jem za ustvarjanje ta API NE bo samodejno ustvaril uporabniških objektov v našem sistemu, če je podan e-poštni naslov.
- Komentarje, posodobljene prek tega API-ja, je mogoče po želji še vedno preveriti za neželeno pošto.
- Konfiguracije, kot je največja dolžina komentarja, če so nastavljene prek administratorske strani Customization Rule, bodo veljale tukaj.
- Če želite uporabnikom omogočiti posodobitev besedila komentarja, lahko v telesu zahteve preprosto določite `comment`. Samodejno bomo ustvarili ustrezni `commentHTML`.
  - Če določite oba, `comment` in `commentHTML`, HTML ne bomo samodejno ustvarili.
  - Če uporabnik v novo besedilo doda omembe ali hashtage, bo to še vedno obdelano kot pri `POST` API-ju.
- Pri posodabljanju `commenterEmail` na komentarju je najbolje tudi navesti `userId`. V nasprotnem primeru morate zagotoviti, da uporabnik s tem e-poštnim naslovom pripada vašemu tenant, sicer bo zahteva zavrnjena.  
- Če je ciljni komentar zaklenjen (`isLocked: true`), je zahteva zavrnjena s `code: 'locked'`. Najprej odklenejte komentar, ga posodobite, nato ga po potrebi ponovno zaklenite.


[inline-code-attrs-start title = 'Minimalen primer PATCH cURL za komentar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH zahteve za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Uporabnik, ki izvaja posodobitev. Po želji se lahko uporabi za preverjanje, ali lahko ureja komentar.  **/
    contextUserId?: string
	/** Ali naj preverimo, ali nov komentar izgleda kot neželena pošta?  **/
    doSpamCheck?: 'true' | 'false'
	/** Ali naj se komentar prikaže "v živo" uporabnikom, ki si ogledujejo primere widgeta za komentarje z istim urlId. OPOMBA: Podvoji strošek kreditov s 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora PATCH za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Vključeno ob neuspehu. **/
    reason?: string
}
[inline-code-end]

---