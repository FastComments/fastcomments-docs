Den eneste struktur sendt via webhooks er WebhookComment-objektet, beskrevet i TypeScript nedenfor.

#### WebhookComment-objektets struktur

##### Strukturen for "create"-begivenheden
Requestbodyen for "create"-begivenheden er et WebhookComment-objekt.

##### Strukturen for "update"-begivenheden
Requestbodyen for "update"-begivenheden er et WebhookComment-objekt.

##### Strukturen for "delete"-begivenheden
Requestbodyen for "delete"-begivenheden er et WebhookComment-objekt.

    Ændring pr. 14. nov. 2023
    Tidligere indeholdt requestbodyen for "delete"-begivenheden kun kommentar-id'et. Den indeholder nu hele kommentaren på tidspunktet for sletningen.


[inline-code-attrs-start title = 'WebhookComment-objektet'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Id'et på kommentaren. **/
    id: string
    /** Id'et eller URL'en, der identificerer kommentartråden. Normaliseret. **/
    urlId: string
    /** URL'en, der peger på hvor kommentaren blev efterladt. **/
    url?: string
    /** Bruger-id'et der skrev kommentaren. Hvis SSO, præfikset med tenant-id. **/
    userId?: string
    /** Emailen på brugeren, der skrev kommentaren. **/
    commenterEmail?: string
    /** Navnet på brugeren, som vises i kommentarboksen. Ved SSO kan det være displayName. **/
    commenterName: string
    /** Rå kommentartekst. **/
    comment: string
    /** Kommentartekst efter parsing. **/
    commentHTML: string
    /** Eksternt id for kommentaren. **/
    externalId?: string
    /** Id'et på forældrekommentaren. **/
    parentId?: string | null
    /** UTC-datoen da kommentaren blev skrevet. **/
    date: UTC_ISO_DateString
    /** Kombineret karma (op - ned) fra stemmerne. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Sandt hvis brugeren var logget ind, da de kommenterede, eller hvis de verificerede kommentaren, eller hvis de bekræftede deres session, da kommentaren blev efterladt. **/
    verified: boolean
    /** Dato for hvornår kommentaren blev verificeret. **/
    verifiedDate?: number
    /** Hvis en moderator markerede kommentaren som gennemgået. **/
    reviewed: boolean
    /** Placeringen, eller base64-enkodningen, af avataren. Vil kun være base64 hvis det var den værdi, der blev sendt med SSO. **/
    avatarSrc?: string
    /** Blev kommentaren manuelt eller automatisk markeret som spam? **/
    isSpam: boolean
    /** Blev kommentaren automatisk markeret som spam? **/
    aiDeterminedSpam: boolean
    /** Er der billeder i kommentaren? **/
    hasImages: boolean
    /** Sidenummeret kommentaren er på for sorteringsretningen "Most Relevant". **/
    pageNumber: number
    /** Sidenummeret kommentaren er på for sorteringsretningen "Oldest First". **/
    pageNumberOF: number
    /** Sidenummeret kommentaren er på for sorteringsretningen "Newest First". **/
    pageNumberNF: number
    /** Blev kommentaren godkendt automatisk eller manuelt? **/
    approved: boolean
    /** Lokale-koden (format: en_us) for brugeren da kommentaren blev skrevet. **/
    locale: string
    /** De @mentions skrevet i kommentaren, som blev korrekt fortolket. **/
    mentions?: CommentUserMention[]
    /** Domænet kommentaren kommer fra. **/
    domain?: string
    /** Den valgfrie liste over moderationsgruppe-id'er forbundet med denne kommentar. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Når brugere tagges i en kommentar, gemmes oplysningerne i en liste kaldet `mentions`. Hvert objekt i den liste
har følgende struktur.

[inline-code-attrs-start title = 'Webhook Mentions-objektet'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Brugerens id. For SSO-brugere vil dette have dit tenant-id som præfiks. **/
    id: string
    /** Den endelige @mention-tagtekst, inklusive @-symbolet. **/
    tag: string
    /** Den oprindelige @mention-tagtekst, inklusive @-symbolet. **/
    rawTag: string
    /** Hvilken type bruger der blev tagget. user = FastComments.com-konto. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Hvis brugeren fravælger notifikationer, vil dette stadig være sat til true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP-metoder

Du kan konfigurere HTTP-metoden for hver webhook-begivenhedstype i administrationspanelet:

- **Oprettelsesbegivenhed**: POST eller PUT (standard: PUT)
- **Opdateringsbegivenhed**: POST eller PUT (standard: PUT)
- **Sletningsbegivenhed**: DELETE, POST, eller PUT (standard: DELETE)

Da alle forespørgsler indeholder et ID, er Create- og Update-operationer idempotente som standard (PUT). Gentagelse af samme Create- eller Update-forespørgsel bør ikke oprette dublerede objekter på din side.

#### Anmodningsoverskrifter

Hver webhook-forespørgsel inkluderer følgende headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Din API-secret |
| `X-FastComments-Timestamp` | Unix-timestamp (sekunder) da forespørgslen blev underskrevet |
| `X-FastComments-Signature` | HMAC-SHA256-signatur (`sha256=<hex>`) |

Se [Sikkerhed & API Tokens](/guides/webhooks/webhooks-api-tokens) for oplysninger om verifikation af HMAC-signaturen.

---