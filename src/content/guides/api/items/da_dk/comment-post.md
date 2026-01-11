[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Dette API-endpoint giver mulighed for at oprette kommentarer.

Almindelige anvendelsestilfælde er brugerdefinerede UI'er, integrationer eller imports.

Bemærkninger:

- Dette API kan opdatere kommentarwidgeten "live", hvis ønsket (dette øger `creditsCost` fra `1` til `2`).
- Dette API vil automatisk oprette brugerobjekter i vores system, hvis der angives en e-mail.
- Forsøg på at gemme to kommentarer med forskellige e-mails, men samme brugernavn, vil resultere i en fejl for den anden kommentar. 
- Hvis du angiver `parentId`, og en børnekommentar har `notificationSentForParent` sat til false, **vil vi sende notifikationer for forældrekommentaren**. Dette gøres hver time (vi samler notifikationerne for at reducere antallet af sendte e-mails).
- Hvis du vil sende velkomst-e-mails ved oprettelse af brugere, eller e-mails til verifikation af kommentarer, sæt `sendEmails` til `true` i forespørgselsparametrene.
- Kommentarer oprettet via dette API vil blive vist på Analytics- og Moderation-siderne i admin-appen.
- "bad words" bliver stadig maskerede i kommentatornavne og kommentartekst, hvis indstillingen er slået til.
- Kommentarer oprettet via dette API kan stadig blive tjekket for spam, hvis ønsket.
- Konfiguration såsom maksimal kommentar-længde, hvis konfigureret via Customization Rule-adminsiden, vil gælde her.

De minimale data, der kræves for at indsende og som vil blive vist i kommentar-widgeten, er følgende:

[inline-code-attrs-start title = 'Minimum Kommentar POST cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

En mere realistisk forespørgsel kan se sådan ud:

[inline-code-attrs-start title = 'Kommentar POST cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Kommentar POST-forespørgselsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Om kommentaren skal vises "live" for brugere, der ser instanser af kommentarwidgeten med samme urlId. BEMÆRK: Fordobler kreditforbruget fra 1 til 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Kommentar POST-responsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Medtages ved fejl. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Medtages ved fejl. **/
    reason?: string
    /** Den oprettede kommentar. **/
    comment?: Comment
    /** Den tilknyttede bruger, som måske allerede fandtes eller ej. **/
    user?: User
}
[inline-code-end]