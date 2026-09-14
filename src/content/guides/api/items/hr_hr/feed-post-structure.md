A `FeedPost` objekt predstavlja objavu u FastComments feedu. Feed je tok objava sa svojim vlastitim nitima komentara, prikazan od strane Feed widgeta. Svaka objava ima autora, opcionalni bogati sadržaj, medije i linkove, i može biti označena tagovima kako bi se feed mogao filtrirati.

Struktura za `FeedPost` objekt je sljedeća:

[inline-code-attrs-start title = 'Struktura FeedPosta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** ID FastComments ili SSO korisnika koji je autor objave. **/
    fromUserId?: string
    /** Popunjeno iz korisnika kada nije postavljeno. **/
    fromUserDisplayName?: string | null
    /** READONLY. Popunjeno iz korisnika. **/
    fromUserAvatar?: string | null
    /** Koristi se za filtriranje feeda. **/
    tags?: string[]
    /** Težina sortiranja unutar feeda. Veće vrijednosti se sortiraju prve. **/
    weight?: number
    /** Parovi ključ/vrijednost slobodnog oblika za vlastitu upotrebu. **/
    meta?: Record<string, string>
    /** Sanitizirani HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Vrsta reakcije za brojanje. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Gdje medijski element vodi kada se klikne. **/
    linkUrl?: string
    /** Jedan unos po izvedbi. Widget odabire najbolje. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Tekst linka, npr. "Sign up now". **/
    text?: string
    /** Naslov prikazan uz link. **/
    title?: string
    /** Opis prikazan uz link. **/
    description?: string
    url?: string
}
[inline-code-end]

Napomene:

- Neka od ovih polja su označena `READONLY` - ona se vraćaju putem API-ja, ali se ne mogu postaviti.
- Komentari na objavi su regularni komentari čiji je `urlId` `post:` praćen ID-om objave `_id`. Upotrijebite tu vrijednost s Comment API-jem za čitanje ili stvaranje komentara na objavi.