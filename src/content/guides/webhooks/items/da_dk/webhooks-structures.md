Den eneste struktur, der sendes via webhooks, er WebhookComment-objektet, beskrevet i TypeScript nedenfor.

#### WebhookComment-objektets struktur

##### Strukturen for "Create"-hændelsen
Request body for "create"-hændelsen er et WebhookComment-objekt.

##### Strukturen for "Update"-hændelsen
Request body for "update"-hændelsen er et WebhookComment-objekt.

##### Strukturen for "Delete"-hændelsen
Request body for "delete"-hændelsen er et WebhookComment-objekt.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'WebhookComment-objektet'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Id'et for kommentaren. **/
    id: string
    /** Id'et eller URL'en, der identificerer kommentartråden. Normaliseret. **/
    urlId: string
    /** URL'en, der peger på, hvor kommentaren blev efterladt. **/
    url?: string
    /** Bruger-id'et, der skrev kommentaren. Hvis SSO, præfikset med tenant-id. **/
    userId?: string
    /** Brugerens e-mail, der skrev kommentaren. **/
    commenterEmail?: string
    /** Navnet på brugeren, der vises i kommentar-widget'en. Ved SSO kan det være displayName. **/
    commenterName: string
    /** Rå kommentartekst. **/
    comment: string
    /** Kommentartekst efter parsing. **/
    commentHTML: string
    /** Eksternt id for kommentaren. **/
    externalId?: string
    /** Id'et på forældrekommentaren. **/
    parentId?: string | null
    /** UTC-datoen for, hvornår kommentaren blev skrevet. **/
    date: UTC_ISO_DateString
    /** Kombinationen af karma (op - ned) fra stemmer. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Sandt hvis brugeren var logget ind, da de kommenterede, eller hvis de verificerede kommentaren, eller hvis de verificerede deres session, da kommentaren blev skrevet. **/
    verified: boolean
    /** Datoen for, hvornår kommentaren blev verificeret. **/
    verifiedDate?: number
    /** Hvis en moderator markerede kommentaren som gennemgået. **/
    reviewed: boolean
    /** Placeringen, eller base64-encoding, af avatar. Vil kun være base64, hvis det var værdien sendt med SSO. **/
    avatarSrc?: string
    /** Blev kommentaren markeret som spam manuelt eller automatisk? **/
    isSpam: boolean
    /** Blev kommentaren automatisk markeret som spam? **/
    aiDeterminedSpam: boolean
    /** Er der billeder i kommentaren? **/
    hasImages: boolean
    /** Sidenummeret hvor kommentaren er for "Most Relevant" sorteringsretning. **/
    pageNumber: number
    /** Sidenummeret hvor kommentaren er for "Oldest First" sorteringsretning. **/
    pageNumberOF: number
    /** Sidenummeret hvor kommentaren er for "Newest First" sorteringsretning. **/
    pageNumberNF: number
    /** Blev kommentaren godkendt automatisk eller manuelt? **/
    approved: boolean
    /** Lokalekoden (format: en_us) for brugeren, da kommentaren blev skrevet. **/
    locale: string
    /** De @mentions skrevet i kommentaren, som blev fortolket. **/
    mentions?: CommentUserMention[]
    /** Domænet kommentaren kommer fra. **/
    domain?: string
    /** Den valgfrie liste af moderationsgruppe-id'er knyttet til denne kommentar. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Når brugere tagges i en kommentar, gemmes oplysningerne i en liste kaldet `mentions`. Hvert objekt i den liste
har følgende struktur.

[inline-code-attrs-start title = 'Webhook Mentions-objektet'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Bruger-id'et. For SSO-brugere vil dette have jeres tenant-id som præfiks. **/
    id: string
    /** Den endelige @mention-tagtekst, inklusiv @-symbolet. **/
    tag: string
    /** Den originale @mention-tagtekst, inklusiv @-symbolet. **/
    rawTag: string
    /** Hvilken type bruger blev tagget. user = FastComments.com-konto. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Hvis brugeren fravælger notifikationer, vil dette stadig være sat til true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP-metoder i brug

**Create og Update bruger begge HTTP PUT og ikke POST!**

Da alle vores requests indeholder et ID, bør gentagelse af samme Create- eller Update-request ikke oprette nye objekter på jeres side.

Det betyder, at disse kald er idempotente og bør være PUT-hændelser i henhold til HTTP-specifikationen.

---