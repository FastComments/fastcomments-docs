[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava stvaranje komentara.

Uobičajeni slučajevi upotrebe su prilagođena korisnička sučelja, integracije ili uvozi.

Napomene:

- Ovaj API može po želji ažurirati widget za komentare "uživo" (to povećava `creditsCost` s `1` na `2`).
- Ovaj API će automatski stvoriti korisničke objekte u našem sustavu ako je naveden email.
- Pokušaj spremanja dva komentara s različitim emailovima, ali istim korisničkim imenom, rezultirat će greškom za drugi komentar.
- Ako navodite `parentId`, a podređeni komentar ima `notificationSentForParent` postavljeno na false, **poslat ćemo obavijesti za nadređeni komentar**. To se radi svaki sat (obavijesti grupiramo kako bismo smanjili broj poslanih emailova).
- Ako želite slati dobrodošlice prilikom stvaranja korisnika, ili emailove za potvrdu komentara, postavite `sendEmails` na `true` u query parametrima.
- Komentari stvoreni putem ovog API-ja prikazat će se na stranicama Analytics i Moderation u admin aplikaciji.
- "loše riječi" i dalje se maskiraju u imenima komentatora i tekstu komentara ako je ta postavka uključena.
- Komentari stvoreni putem ovog API-ja mogu se po želji i dalje provjeravati na spam.
- Konfiguracija poput maksimalne duljine komentara, ako je postavljena putem admin stranice Customization Rule, primjenjuje se i ovdje.

Minimalni podaci potrebni za slanje koji će se prikazati u widgetu za komentare su sljedeći:

[inline-code-attrs-start title = 'Primjer minimalnog POST cURL zahtjeva za komentar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Realističniji zahtjev može izgledati ovako:

[inline-code-attrs-start title = 'Primjer POST cURL zahtjeva za komentar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura POST zahtjeva za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Treba li se komentar prikazati "uživo" korisnicima koji gledaju instance widgeta za komentare s istim urlId. NAPOMENA: Udvostručuje cijenu kredita s 1 na 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura POST odgovora za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    /** Stvoreni komentar. **/
    comment?: Comment
    /** Pridruženi korisnik, koji možda već postoji ili ne. **/
    user?: User
}
[inline-code-end]