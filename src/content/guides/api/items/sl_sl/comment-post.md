[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ta končna točka API omogoča ustvarjanje komentarjev.

Pogoste uporabe so prilagojeni uporabniški vmesniki, integracije ali uvozi.

Opombe:

- Ta API lahko po želji posodobi prikazovalnik komentarjev "v živo" (to poveča `creditsCost` z `1` na `2`).
- Ta API bo samodejno ustvaril objekte uporabnikov v našem sistemu, če je podan e-poštni naslov.
- Poskus shranjevanja dveh komentarjev z različnimi e-poštnimi naslovi, vendar istim uporabniškim imenom, bo pri drugem komentarju povzročil napako.
- Če določite `parentId`, in ima otroški komentar `notificationSentForParent` nastavljen na false, **bomo poslali obvestila za starševski komentar**. To se izvaja vsako uro (obvestila zbiramo skupaj, da zmanjšamo število poslanih e-poštnih sporočil).
- Če želite poslati dobrodošla e-poštna sporočila ob ustvarjanju uporabnikov ali e-poštna sporočila za preverjanje komentarjev, v poizvedbenih parametrih nastavite `sendEmails` na `true`.
- Komentarji ustvarjeni preko tega API se bodo prikazali na straneh Analitike in Moderacije v administrativni aplikaciji.
- "bad words" so še vedno zastrti v imenih komentatorjev in besedilu komentarjev, če je nastavitev vklopljena.
- Komentarje ustvarjene preko tega API-ja je mogoče še vedno preveriti za neželeno pošto, če želite.
- Konfiguracija, kot je največja dolžina komentarja, če je nastavljena preko strani pravil za prilagoditev (Customization Rule) v administraciji, se uporabi tukaj.

Najmanjši podatki, potrebni za oddajo in prikaz v pripomočku za komentarje, so naslednji:

[inline-code-attrs-start title = 'Minimalni primer POST cURL za komentar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

Bolj realističen primer zahteve je:

[inline-code-attrs-start title = 'Primer POST cURL za komentar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura POST zahtevka za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Ali naj se komentar prikaže "v živo" uporabnikom, ki si ogledajo primere pripomočka za komentarje z istim urlId. OPOZORILO: Podvoji strošek kreditov s 1 na 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora POST za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Vključeno ob napaki. **/
    reason?: string
    /** Ustvarjeni komentar. **/
    comment?: Comment
    /** Povezani uporabnik, ki je morda že obstajal ali pa še ne. **/
    user?: User
}
[inline-code-end]