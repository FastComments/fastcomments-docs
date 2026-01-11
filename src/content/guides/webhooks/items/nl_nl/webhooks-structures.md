De enige structuur die via webhooks wordt verzonden is het WebhookComment-object, hieronder in TypeScript uiteengezet.

#### De WebhookComment Object-structuur

##### Structuur van het "create"-event
De request body van het "create" event is een WebhookComment-object.

##### Structuur van het "update"-event
De request body van het "update" event is een WebhookComment-object.

##### Structuur van het "delete"-event
De request body van het "delete" event is een WebhookComment-object.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Het WebhookComment-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** De id van de reactie. **/
    id: string
    /** De id of URL die de reactiedraad identificeert. Genormaliseerd. **/
    urlId: string
    /** De URL die verwijst naar waar de reactie is achtergelaten. **/
    url?: string
    /** De gebruiker-id die de reactie plaatste. Bij SSO voorafgegaan door de tenant-id. **/
    userId?: string
    /** Het e-mailadres van de gebruiker die de reactie achterliet. **/
    commenterEmail?: string
    /** De naam van de gebruiker die in de comment-widget wordt weergegeven. Bij SSO kan dit displayName zijn. **/
    commenterName: string
    /** Ruwe reactietekst. **/
    comment: string
    /** Reactietekst na parsing. **/
    commentHTML: string
    /** Externe id van de reactie. **/
    externalId?: string
    /** De id van de bovenliggende reactie. **/
    parentId?: string | null
    /** De UTC-datum waarop de reactie is geplaatst. **/
    date: UTC_ISO_DateString
    /** Gecombineerde karma (up - down) van stemmen. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Waar als de gebruiker was ingelogd toen hij reageerde, of als de reactie geverifieerd is, of als de sessie van de gebruiker geverifieerd was toen de reactie werd geplaatst. **/
    verified: boolean
    /** Datum waarop de reactie is geverifieerd. **/
    verifiedDate?: number
    /** Of een moderator de reactie als beoordeeld heeft gemarkeerd. **/
    reviewed: boolean
    /** De locatie, of base64-encoding, van de avatar. Alleen base64 als dat de waarde was die bij SSO werd doorgegeven. **/
    avatarSrc?: string
    /** Is de reactie handmatig of automatisch als spam gemarkeerd? **/
    isSpam: boolean
    /** Is de reactie automatisch als spam gemarkeerd? **/
    aiDeterminedSpam: boolean
    /** Bevat de reactie afbeeldingen? **/
    hasImages: boolean
    /** Het paginanummer waarop de reactie staat voor de sorteer richting "Most Relevant". **/
    pageNumber: number
    /** Het paginanummer waarop de reactie staat voor de sorteer richting "Oldest First". **/
    pageNumberOF: number
    /** Het paginanummer waarop de reactie staat voor de sorteer richting "Newest First". **/
    pageNumberNF: number
    /** Is de reactie automatisch of handmatig goedgekeurd? **/
    approved: boolean
    /** De locale-code (format: en_us) van de gebruiker toen de reactie werd geschreven. **/
    locale: string
    /** De @mentions in de reactie die succesvol zijn geparseerd. **/
    mentions?: CommentUserMention[]
    /** Het domein waar de reactie vandaan komt. **/
    domain?: string
    /** De optionele lijst met moderatiegroep-ids die aan deze reactie zijn gekoppeld. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Wanneer gebruikers in een reactie worden getagd, wordt de informatie opgeslagen in een lijst genaamd `mentions`. Elk object in die lijst
heeft de volgende structuur.

[inline-code-attrs-start title = 'Het Webhook Vermeldingen-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** De gebruiker-id. Voor SSO-gebruikers wordt deze voorafgegaan door uw tenant-id. **/
    id: string
    /** De uiteindelijke @mention-tagtekst, inclusief het @-symbool. **/
    tag: string
    /** De oorspronkelijke @mention-tagtekst, inclusief het @-symbool. **/
    rawTag: string
    /** Welk type gebruiker werd getagd. user = FastComments.com-account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Als de gebruiker zich afmeldt voor notificaties, blijft dit toch op true staan. **/
    sent: boolean
}
[inline-code-end]

#### HTTP-methoden

U kunt de HTTP-methode voor elk type webhook-gebeurtenis configureren in het beheerderspaneel:

- **Create Event**: POST of PUT (standaard: PUT)
- **Update Event**: POST of PUT (standaard: PUT)
- **Delete Event**: DELETE, POST of PUT (standaard: DELETE)

Omdat alle requests een ID bevatten, zijn Create- en Update-bewerkingen standaard idempotent (PUT). Het opnieuw versturen van dezelfde Create- of Update-request zou geen dubbele objecten aan uw zijde mogen aanmaken.

#### Request Headers

Elke webhook-request bevat de volgende headers:

| Header | Beschrijving |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Uw API Secret |
| `X-FastComments-Timestamp` | Unix-tijdstempel (seconden) waarop het verzoek werd ondertekend |
| `X-FastComments-Signature` | HMAC-SHA256-handtekening (`sha256=<hex>`) |

Zie [Security & API Tokens](/guides/webhooks/webhooks-api-tokens) voor informatie over het verifiëren van de HMAC-handtekening.