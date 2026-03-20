De enige structuur die via webhooks wordt verzonden is het WebhookComment-object, hieronder in TypeScript uiteengezet.

#### De structuur van het WebhookComment-object

##### De structuur van het "Create"-event
Het request body van het "create" event is een WebhookComment-object.

##### De structuur van het "Update"-event
Het request body van het "update" event is een WebhookComment-object.

##### De structuur van het "Delete"-event
Het request body van het "delete" event is een WebhookComment-object.

    Wijziging per 14 nov 2023
    Voorheen bevatte het request body van het "delete" event alleen de comment id. Het bevat nu de volledige opmerking op het moment van verwijdering.


[inline-code-attrs-start title = 'Het WebhookComment-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** De id van de opmerking. **/
    id: string
    /** De id of URL die de commentdraad identificeert. Genormaliseerd. **/
    urlId: string
    /** De URL die wijst naar waar de opmerking is achtergelaten. **/
    url?: string
    /** De gebruikers-id die de opmerking heeft geplaatst. Bij SSO voorafgegaan door de tenant-id. **/
    userId?: string
    /** Het e-mailadres van de gebruiker die de opmerking plaatste. **/
    commenterEmail?: string
    /** De naam van de gebruiker die in de comment-widget wordt weergegeven. Bij SSO kan dit displayName zijn. **/
    commenterName: string
    /** Ruwe tekst van de opmerking. **/
    comment: string
    /** Tekst van de opmerking na verwerking. **/
    commentHTML: string
    /** Externe id van de opmerking. **/
    externalId?: string
    /** De id van de bovenliggende opmerking. **/
    parentId?: string | null
    /** De UTC-datum waarop de opmerking is geplaatst. **/
    date: UTC_ISO_DateString
    /** Gecombineerde karma (up - down) van stemmen. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Waar als de gebruiker was ingelogd toen hij reageerde, of als hij de opmerking heeft geverifieerd, of als hij zijn sessie had geverifieerd toen de opmerking werd geplaatst. **/
    verified: boolean
    /** Datum waarop de opmerking is geverifieerd. **/
    verifiedDate?: number
    /** Of een moderator de opmerking als beoordeeld heeft gemarkeerd. **/
    reviewed: boolean
    /** De locatie, of base64-codering, van de avatar. Zal alleen base64 zijn als dat de waarde was die bij SSO werd meegegeven. **/
    avatarSrc?: string
    /** Is de opmerking handmatig of automatisch als spam gemarkeerd? **/
    isSpam: boolean
    /** Is de opmerking automatisch als spam gemarkeerd? **/
    aiDeterminedSpam: boolean
    /** Bevat de opmerking afbeeldingen? **/
    hasImages: boolean
    /** Het paginanummer waarop de opmerking staat voor de sorteerrichting "Most Relevant". **/
    pageNumber: number
    /** Het paginanummer waarop de opmerking staat voor de sorteerrichting "Oldest First". **/
    pageNumberOF: number
    /** Het paginanummer waarop de opmerking staat voor de sorteerrichting "Newest First". **/
    pageNumberNF: number
    /** Is de opmerking automatisch of handmatig goedgekeurd? **/
    approved: boolean
    /** De lococode (formaat: en_us) van de gebruiker toen de opmerking werd geschreven. **/
    locale: string
    /** De @mentions in de opmerking die succesvol zijn geparsed. **/
    mentions?: CommentUserMention[]
    /** Het domein waar de opmerking vandaan komt. **/
    domain?: string
    /** De optionele lijst met moderatiegroep-ids die aan deze opmerking zijn gekoppeld. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Wanneer gebruikers worden getagd in een opmerking, wordt die informatie opgeslagen in een lijst genaamd `mentions`. Elk object in die lijst
heeft de volgende structuur.

[inline-code-attrs-start title = 'Het Webhook Mentions-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** De gebruikers-id. Voor SSO-gebruikers wordt hier uw tenant-id aan voorafgegaan. **/
    id: string
    /** De uiteindelijke @mention-tagtekst, inclusief het @-symbool. **/
    tag: string
    /** De oorspronkelijke @mention-tagtekst, inclusief het @-symbool. **/
    rawTag: string
    /** Welk type gebruiker werd genoemd. user = FastComments.com-account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Als de gebruiker zich afmeldt voor meldingen, blijft dit toch op true staan. **/
    sent: boolean
}
[inline-code-end]

#### HTTP-methoden

U kunt de HTTP-methode voor elk webhook-eventtype configureren in het beheerpaneel:

- **Create Event**: POST of PUT (standaard: PUT)
- **Update Event**: POST of PUT (standaard: PUT)
- **Delete Event**: DELETE, POST of PUT (standaard: DELETE)

Aangezien alle verzoeken een ID bevatten, zijn Create- en Update-bewerkingen standaard idempotent (PUT). Het herhalen van hetzelfde Create- of Update-verzoek zou geen dubbele objecten aan uw kant moeten aanmaken.

#### Request Headers

Elk webhook-verzoek bevat de volgende headers:

| Header | Beschrijving |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Your API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256-handtekening (`sha256=<hex>`) |

Zie [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) voor informatie over het verifiëren van de HMAC-handtekening.