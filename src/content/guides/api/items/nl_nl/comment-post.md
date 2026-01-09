[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Dit API-eindpunt biedt de mogelijkheid om reacties te maken.

Veelvoorkomende gebruikssituaties zijn aangepaste UIs, integraties of imports.

Opmerkingen:

- Deze API kan het reactiewidget "live" bijwerken indien gewenst (dit verhoogt de `creditsCost` van `1` naar `2`).
- Deze API maakt automatisch gebruikersobjecten in ons systeem aan als een e-mailadres wordt opgegeven.
- Poging om twee reacties op te slaan met verschillende e-mails, maar dezelfde gebruikersnaam, resulteert in een fout voor de tweede reactie. 
- Als u `parentId` opgeeft, en een kindreactie heeft `notificationSentForParent` als false, **zullen wij meldingen voor de ouderreactie verzenden**. Dit gebeurt elk uur (we groeperen de meldingen om het aantal verzonden e-mails te verminderen).
- Als u welkomst-e-mails wilt verzenden bij het aanmaken van gebruikers, of e-mails voor reactie-verificatie, zet `sendEmails` op `true` in de queryparameters.
- Reacties die via deze API worden gemaakt verschijnen in de Analytics- en Moderation-pagina's van de beheerapp.
- "ongepaste woorden" blijven nog steeds gemaskeerd in de namen van reageerders en de reactietekst als de instelling is ingeschakeld.
- Reacties gemaakt via deze API kunnen desgewenst nog steeds op spam worden gecontroleerd.
- Configuraties zoals maximale reactielengte, indien geconfigureerd via de Customization Rule-beheerpagina, zijn hier van toepassing.

De minimale gegevens die vereist zijn om te verzenden en die in het reactiewidget worden weergegeven, zijn als volgt:

[inline-code-attrs-start title = 'Minimaal Comment POST cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Een realistischer verzoek kan er als volgt uitzien:

[inline-code-attrs-start title = 'Comment POST cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structuur van Comment POST-verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Of de reactie "live" moet verschijnen voor gebruikers die instanties van het reactiewidget met dezelfde urlId bekijken. OPMERKING: Verdubbelt de credits-kosten van 1 naar 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Comment POST-respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Opgenomen bij mislukking. **/
    reason?: string
    /** De aangemaakte reactie. **/
    comment?: Comment
    /** De bijbehorende gebruiker, die mogelijk al bestond of niet. **/
    user?: User
}
[inline-code-end]

---