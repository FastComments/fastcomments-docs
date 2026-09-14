A `FeedPost` objekat predstavlja objavu u FastComments feedu. Feed je tok objava sa svojim nitima komentara, prikazan od strane Feed widgeta. Svaka objava ima autora, opcioni bogati sadržaj, medije i linkove, i može biti označena tako da se feed može filtrirati.

Struktura za `FeedPost` objekat je sledeća:

[inline-code-attrs-start title = 'Struktura FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** ID FastComments ili SSO korisnika koji je autor objave. **/
    fromUserId?: string
    /** Popunjava se iz podataka korisnika ako nije postavljeno. **/
    fromUserDisplayName?: string | null
    /** READONLY. Popunjava se iz podataka korisnika. **/
    fromUserAvatar?: string | null
    /** Koristi se za filtriranje feeda. **/
    tags?: string[]
    /** Težina sortiranja unutar feeda. Veće vrednosti se sortiraju prve. **/
    weight?: number
    /** Slobodni parovi ključ/vrednost za vašu ličnu upotrebu. **/
    meta?: Record<string, string>
    /** Sanitisani HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Tip reakcije za brojanje. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Gde medijski element vodi kada se klikne. **/
    linkUrl?: string
    /** Jedan unos po rendiciji. Widget bira najbolje. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Tekst linka, npr. "Registrujte se sada". **/
    text?: string
    /** Naslov prikazan uz link. **/
    title?: string
    /** Opis prikazan uz link. **/
    description?: string
    url?: string
}
[inline-code-end]

Napomene:

- Neka od ovih polja su označena `READONLY` – ona se vraćaju iz API‑ja, ali se ne mogu postaviti.
- Komentari na objavi su regularni komentari čiji je `urlId` `post:` praćen ID‑jem objave `_id`. Koristite tu vrednost sa Comment API‑jem da biste čitali ili kreirali komentare na objavi.