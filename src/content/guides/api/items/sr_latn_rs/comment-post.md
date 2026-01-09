[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava kreiranje komentara.

Uobičajeni slučajevi upotrebe su prilagođeni UI-ji, integracije, ili uvozi.

Napomene:

- Ovaj API može ažurirati widget za komentare „u realnom vremenu“ ako se želi (ovo povećava `creditsCost` sa `1` na `2`).
- Ovaj API će automatski kreirati objekte korisnika u našem sistemu ako je naveden email.
- Pokušaj da se sačuvaju dva komentara sa različitim emailovima, ali istim korisničkim imenom, rezultiraće greškom za drugi komentar. 
- Ako specificirate `parentId`, i ukoliko dečiji komentar ima `notificationSentForParent` postavljeno na false, **poslaćemo notifikacije za roditeljski komentar**. Ovo se radi na svakih sat vremena (grupisemo notifikacije kako bismo smanjili broj poslatih emailova).
- Ako želite slati welcome email-ove prilikom kreiranja korisnika, ili email-ove za verifikaciju komentara, postavite `sendEmails` na `true` u query parametrima.
- Komentari kreirani preko ovog API-ja će se pojaviti na stranicama Analitike i Moderacije u admin aplikaciji.
- "neprimerene reči" su i dalje maskirane u imenima komentatora i tekstu komentara ako je podešavanje uključeno.
- Komentari kreirani putem ovog API-ja i dalje mogu biti provereni za spam ako je potrebno.
- Konfiguracije kao što je maksimalna dužina komentara, ukoliko su podešene preko admin stranice Customization Rule, važiće ovde.

Minimalni podaci potrebni za slanje koji će se prikazati u widgetu za komentare su sledeći:

[inline-code-attrs-start title = 'Minimalni cURL POST primer za komentar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Realniji zahtev može izgledati ovako:

[inline-code-attrs-start title = 'cURL POST primer za komentar'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura POST zahteva za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Da li komentar treba da se pojavi "uživo" korisnicima koji gledaju instance widgeta za komentare sa istim urlId. NAPOMENA: Duplira trošak kredita sa 1 na 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora POST zahteva za komentar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Uključeno u slučaju greške. **/
    reason?: string
    /** Kreirani komentar. **/
    comment?: Comment
    /** Povezani korisnik, koji je možda već postojao ili možda nije. **/
    user?: User
}
[inline-code-end]