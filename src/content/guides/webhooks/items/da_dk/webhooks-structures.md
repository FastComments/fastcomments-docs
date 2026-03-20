Den eneste struktur, der sendes via webhooks, er WebhookComment-objektet, beskrevet i TypeScript nedenfor.

#### WebhookComment-objektets struktur

##### Struktur for "Create"-hændelsen
Request-body'en for "create"-hændelsen er et WebhookComment-objekt.

##### Struktur for "Update"-hændelsen
Request-body'en for "update"-hændelsen er et WebhookComment-objekt.

##### Struktur for "Delete"-hændelsen
Request-body'en for "delete"-hændelsen er et WebhookComment-objekt.

    Ændring pr. 14. nov. 2023
    Tidligere indeholdt request-body'en for "delete"-hændelsen kun kommentarens id. Den indeholder nu den fulde kommentar på tidspunktet for sletningen.


[inline-code-attrs-start title = 'WebhookComment-objektet'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Kommentarens id. **/
    id: string
    /** Id'et eller URL'en, der identificerer kommentartræet. Normaliseret. **/
    urlId: string
    /** URL'en, der peger på hvor kommentaren blev efterladt. **/
    url?: string
    /** Bruger-id'et for den, der skrev kommentaren. Hvis SSO, er det præfikset med tenant-id. **/
    userId?: string
    /** E-mailen på brugeren, der skrev kommentaren. **/
    commenterEmail?: string
    /** Navnet på brugeren, som vises i kommentærwidget'en. Ved SSO kan det være displayName. **/
    commenterName: string
    /** Rå kommentartekst. **/
    comment: string
    /** Kommentartekst efter parsing. **/
    commentHTML: string
    /** Eksternt id for kommentaren. **/
    externalId?: string
    /** Id'et på forælderkommentaren. **/
    parentId?: string | null
    /** UTC-datoen hvor kommentaren blev skrevet. **/
    date: UTC_ISO_DateString
    /** Kombineret karma (op - ned) af stemmer. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Sandt hvis brugeren var logget ind, da de kommenterede, eller hvis deres kommentar var verificeret, eller hvis de verificerede deres session, da kommentaren blev skrevet. **/
    verified: boolean
    /** Datoen hvor kommentaren blev verificeret. **/
    verifiedDate?: number
    /** Hvis en moderator markerede kommentaren som gennemgået. **/
    reviewed: boolean
    /** Placeringen eller base64-encodningen af avataren. Vil kun være base64 hvis det var den værdi, der blev sendt med SSO. **/
    avatarSrc?: string
    /** Blev kommentaren markeret som spam manuelt eller automatisk? **/
    isSpam: boolean
    /** Blev kommentaren automatisk markeret som spam? **/
    aiDeterminedSpam: boolean
    /** Er der billeder i kommentaren? **/
    hasImages: boolean
    /** Sidetallet kommentaren er på for sorteringsretningen "Most Relevant". **/
    pageNumber: number
    /** Sidetallet kommentaren er på for sorteringsretningen "Oldest First". **/
    pageNumberOF: number
    /** Sidetallet kommentaren er på for sorteringsretningen "Newest First". **/
    pageNumberNF: number
    /** Blev kommentaren godkendt automatisk eller manuelt? **/
    approved: boolean
    /** Sprogkode (format: en_us) for brugeren da kommentaren blev skrevet. **/
    locale: string
    /** De @mentions skrevet i kommentaren, der blev korrekt analyseret. **/
    mentions?: CommentUserMention[]
    /** Domænet kommentaren kommer fra. **/
    domain?: string
    /** Den valgfrie liste af moderation group ids associeret med denne kommentar. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Når brugere tagges i en kommentar, gemmes informationen i en liste kaldet `mentions`. Hvert objekt i den liste
har følgende struktur.

[inline-code-attrs-start title = 'Webhook Mentions-objektet'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Bruger-id'et. For SSO-brugere vil dette være præfikset med dit tenant-id. **/
    id: string
    /** Den endelige @mention-tagtekst, inklusive @-symbolet. **/
    tag: string
    /** Den originale @mention-tagtekst, inklusive @-symbolet. **/
    rawTag: string
    /** Hvilken type bruger der blev tagget. user = FastComments.com-konto. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Hvis brugeren fravælger notifikationer, vil dette stadig være sat til true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP-metoder

Du kan konfigurere HTTP-metoden for hver webhook-hændelsestype i adminpanelet:

- **Create Event**: POST eller PUT (standard: PUT)
- **Update Event**: POST eller PUT (standard: PUT)
- **Delete Event**: DELETE, POST eller PUT (standard: DELETE)

Da alle requests indeholder et ID, er Create- og Update-operationer idempotente som standard (PUT). Gentagelse af samme Create- eller Update-request bør ikke skabe duplikerede objekter hos dig.

#### Request-headere

Hver webhook-request indeholder følgende headere:

| Header | Beskrivelse |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Din API Secret |
| `X-FastComments-Timestamp` | Unix-timestamp (sekunder) hvor requesten blev signeret |
| `X-FastComments-Signature` | HMAC-SHA256-signatur (`sha256=<hex>`) |

Se [Sikkerhed & API Tokens](/guide-webhooks.html#webhooks-api-tokens) for information om verifikation af HMAC-signaturen.