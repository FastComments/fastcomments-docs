The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### De structuur van het WebhookComment-object

##### De structuur van het "Create"-event
De request‑body van het "create"-event is een WebhookComment‑object.

##### De structuur van het "Update"-event
De request‑body van het "update"-event is een WebhookComment‑object.

##### De structuur van het "Delete"-event
De request‑body van het "delete"-event is een WebhookComment‑object.

    Wijziging vanaf 14 november 2023
    Voorheen bevatte de request‑body van het "delete"-event alleen de commentaar‑id. Nu bevat het de volledige commentaar op het moment van verwijdering.

Elke sleutel is altijd aanwezig in de body. Wanneer een commentaar geen waarde heeft voor een veld, draagt de body `null` (of `false` voor booleans en `[]` voor lijsten), zodat de vorm van een levering nooit varieert van het ene commentaar naar het andere.

[inline-code-attrs-start title = 'Het WebhookComment-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** De id van het commentaar. **/
    id: string
    /** De id of URL die de commentaarthread identificeert. Genormaliseerd. **/
    urlId: string
    /** De URL die aangeeft waar het commentaar is geplaatst. **/
    url: string | null
    /** De gebruikers‑id die het commentaar heeft geplaatst. Bij SSO, voorafgegaan door tenant‑id. **/
    userId: string | null
    /** Het e‑mailadres van de gebruiker die het commentaar heeft geplaatst. **/
    commenterEmail: string | null
    /** De naam van de gebruiker die wordt weergegeven in de commentaarwidget. Bij SSO kan dit displayName zijn. **/
    commenterName: string
    /** Ruwe commentaartekst. **/
    comment: string
    /** Commentaartekst na parsing. **/
    commentHTML: string
    /** Externe id van het commentaar. **/
    externalId: string | null
    /** De id van het bovenliggende commentaar. **/
    parentId: string | null
    /** De UTC‑datum waarop het commentaar is geplaatst. **/
    date: UTC_ISO_DateString
    /** Gecombineerde karma (up - down) van stemmen. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Waar als de gebruiker ingelogd was toen hij commentaar gaf, of het commentaar heeft geverifieerd, of zijn sessie heeft geverifieerd toen het commentaar werd geplaatst. **/
    verified: boolean
    /** De UTC‑datum waarop het commentaar is geverifieerd. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Als een moderator het commentaar heeft gemarkeerd als beoordeeld. **/
    reviewed: boolean
    /** De locatie, of base64‑codering, van de avatar. Wordt alleen base64 als dat de waarde was die met SSO werd doorgegeven. **/
    avatarSrc: string | null
    /** Was het commentaar handmatig of automatisch gemarkeerd als spam? **/
    isSpam: boolean
    /** Was het commentaar automatisch gemarkeerd als spam? **/
    aiDeterminedSpam: boolean
    /** Bevat het commentaar afbeeldingen? **/
    hasImages: boolean
    /** Het paginanummer waarop het commentaar staat voor de sorteeroptie "Meest relevant". **/
    pageNumber: number | null
    /** Het paginanummer waarop het commentaar staat voor de sorteeroptie "Oudste eerst". **/
    pageNumberOF: number | null
    /** Het paginanummer waarop het commentaar staat voor de sorteeroptie "Nieuwste eerst". **/
    pageNumberNF: number | null
    /** Was het commentaar automatisch of handmatig goedgekeurd? **/
    approved: boolean
    /** De locale‑code (formaat: en_us) van de gebruiker toen het commentaar werd geschreven. **/
    locale: string | null
    /** De @mentions die in het commentaar zijn geschreven en succesvol zijn geparseerd. Leeg wanneer er geen zijn. **/
    mentions: CommentUserMention[]
    /** Het domein waar het commentaar vandaan komt. **/
    domain: string | null
    /** De moderatie‑groeps‑ids die aan dit commentaar zijn gekoppeld. Leeg wanneer er geen zijn. **/
    moderationGroupIds: string[]
    /** De badges die naast de naam van de commentator worden getoond bij dit commentaar, op het moment dat het event werd aangemaakt. Leeg wanneer er geen zijn. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Wanneer gebruikers in een commentaar worden getagd, wordt de informatie opgeslagen in een lijst genaamd `mentions`. Elk object in die lijst heeft de volgende structuur.

[inline-code-attrs-start title = 'Het Webhook Mentions-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** De gebruikers‑id. Voor SSO‑gebruikers wordt uw tenant‑id ervoor geplaatst. **/
    id: string
    /** De uiteindelijke @mention‑tagtekst, inclusief het @‑symbool. **/
    tag: string
    /** De oorspronkelijke @mention‑tagtekst, inclusief het @‑symbool. **/
    rawTag: string
    /** Welk type gebruiker werd getagd. user = FastComments.com‑account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Als de gebruiker zich afmeldt voor meldingen, blijft dit toch op true staan. **/
    sent: boolean
}
[inline-code-end]

Elk item in de `badges`‑lijst heeft de volgende structuur. Zie de [Badges guide](/guide-badges.html) voor de badge‑typen.

[inline-code-attrs-start title = 'Het Webhook Badge-object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** De id van de badge die aan deze gebruiker is toegekend. **/
    id: string
    /** Het badge‑type. **/
    type: number
    /** De badge‑beschrijving, getoond bij hover over de badge. **/
    description: string
    /** De tekst die op de badge wordt getoond. **/
    displayLabel?: string | null
    /** De afbeelding die voor de badge wordt getoond, indien er een wordt gebruikt. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Een extra CSS‑klasse toegepast op de badge. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP‑methoden

U kunt de HTTP‑methode voor elk webhook‑eventtype configureren in het admin‑paneel:

- **Create‑event**: POST of PUT (standaard: PUT)
- **Update‑event**: POST of PUT (standaard: PUT)
- **Delete‑event**: DELETE, POST of PUT (standaard: DELETE)

Aangezien alle verzoeken een ID bevatten, zijn Create‑ en Update‑operaties standaard idempotent (PUT). Het herhalen van hetzelfde Create‑ of Update‑verzoek mag geen dubbele objecten aan uw kant creëren.

#### Request‑headers

Elk webhook‑verzoek bevat de volgende headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Uw API‑secret |
| `X-FastComments-Timestamp` | Unix‑timestamp (seconden) wanneer het verzoek werd ondertekend |
| `X-FastComments-Signature` | HMAC‑SHA256‑handtekening (`sha256=<hex>`) |

Zie [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) voor informatie over het verifiëren van de HMAC‑handtekening.