De enige structuur die via webhooks wordt verzonden is het WebhookComment-object, hieronder in TypeScript beschreven.

#### De WebhookComment-objectstructuur

##### Structuur van het "create" evenement
De request body van het "create" evenement is een WebhookComment-object.

##### Structuur van het "update" evenement
De request body van het "update" evenement is een WebhookComment-object.

##### Structuur van het "delete" evenement
De request body van het "delete" evenement is een WebhookComment-object.

    Wijziging vanaf 14 november 2023
    Voorheen bevatte de request body van het "delete"-evenement alleen de comment id. Het bevat nu de volledige opmerking op het moment van verwijdering.


[inline-code-attrs-start title = 'Het WebhookComment-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** De id van de opmerking. **/
    id: string
    /** De id of URL die de commentdraad identificeert. Genormaliseerd. **/
    urlId: string
    /** De URL die verwijst naar waar de opmerking is geplaatst. **/
    url?: string
    /** De user id die de opmerking plaatste. Als SSO, voorafgegaan door tenant id. **/
    userId?: string
    /** Het e-mailadres van de gebruiker die de opmerking plaatste. **/
    commenterEmail?: string
    /** De naam van de gebruiker die in de comment-widget wordt weergegeven. Bij SSO kan dit displayName zijn. **/
    commenterName: string
    /** Onbewerkte tekst van de opmerking. **/
    comment: string
    /** Tekst van de opmerking na parsing. **/
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
    /** True als de gebruiker was ingelogd toen hij commentaar gaf, of hun commentaar is geverifieerd, of als ze hun sessie hebben geverifieerd toen de opmerking werd geplaatst. **/
    verified: boolean
    /** Datum waarop de opmerking is geverifieerd. **/
    verifiedDate?: number
    /** Als een moderator de opmerking als beoordeeld heeft gemarkeerd. **/
    reviewed: boolean
    /** De locatie, of base64-encoding, van de avatar. Zal alleen base64 zijn als dat de waarde was die met SSO is doorgegeven. **/
    avatarSrc?: string
    /** Is de opmerking handmatig of automatisch als spam gemarkeerd? **/
    isSpam: boolean
    /** Is de opmerking automatisch als spam gemarkeerd? **/
    aiDeterminedSpam: boolean
    /** Zitten er afbeeldingen in de opmerking? **/
    hasImages: boolean
    /** Het paginanummer waarop de opmerking staat voor de sorteerrichting "Most Relevant". **/
    pageNumber: number
    /** Het paginanummer waarop de opmerking staat voor de sorteerrichting "Oldest First". **/
    pageNumberOF: number
    /** Het paginanummer waarop de opmerking staat voor de sorteerrichting "Newest First". **/
    pageNumberNF: number
    /** Is de opmerking automatisch of handmatig goedgekeurd? **/
    approved: boolean
    /** De locale-code (formaat: en_us) van de gebruiker toen de opmerking werd geschreven. **/
    locale: string
    /** De @mentions in de opmerking die succesvol zijn geparsed. **/
    mentions?: CommentUserMention[]
    /** Het domein waar de opmerking vandaan komt. **/
    domain?: string
    /** De optionele lijst van moderation group ids die aan deze opmerking zijn gekoppeld. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Wanneer gebruikers in een opmerking worden getagd, wordt die informatie opgeslagen in een lijst genaamd `mentions`. Elk object in die lijst
heeft de volgende structuur.

[inline-code-attrs-start title = 'Het Webhook Mentions-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** De user id. Voor SSO-gebruikers wordt hier uw tenant id als prefix gebruikt. **/
    id: string
    /** De uiteindelijke @mention-tagtekst, inclusief het @-symbool. **/
    tag: string
    /** De oorspronkelijke @mention-tagtekst, inclusief het @-symbool. **/
    rawTag: string
    /** Welk type gebruiker werd getagd. user = FastComments.com-account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Als de gebruiker zich afmeldt voor meldingen, blijft dit toch op true gezet. **/
    sent: boolean
}
[inline-code-end]

#### HTTP-methoden die worden gebruikt

**Create en Update gebruiken beide HTTP PUT en niet POST!**

Aangezien al onze requests een ID bevatten, zou het herhalen van dezelfde Create- of Update-request geen nieuwe objecten aan jouw kant moeten aanmaken.

Dit betekent dat deze calls idempotent zijn en als PUT-events moeten worden verstuurd volgens de HTTP-specificatie.

---