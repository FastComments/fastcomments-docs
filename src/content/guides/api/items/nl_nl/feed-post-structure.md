A `FeedPost` object represents a post in a FastComments feed. A feed is a stream of posts with their own comment threads, rendered by the Feed widget. Every post has an author, optional rich content, media, and links, and can be tagged so that a feed can be filtered.

The structure for the `FeedPost` object is as follows:

[inline-code-attrs-start title = 'FeedPost-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** ALLEEN-LEZEN **/
    _id: string
    /** ALLEEN-LEZEN **/
    tenantId: string
    title?: string
    /** De id van de FastComments- of SSO-gebruiker die het bericht heeft geschreven. **/
    fromUserId?: string
    /** Gevuld vanuit de gebruiker wanneer niet ingesteld. **/
    fromUserDisplayName?: string | null
    /** ALLEEN-LEZEN. Gevuld vanuit de gebruiker. **/
    fromUserAvatar?: string | null
    /** Gebruikt om een feed te filteren. **/
    tags?: string[]
    /** Sorteergewicht binnen een feed. Hogere waarden sorteren eerst. **/
    weight?: number
    /** Vrije sleutel/waarde-paren voor eigen gebruik. **/
    meta?: Record<string, string>
    /** Gesaniteerde HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** ALLEEN-LEZEN **/
    createdAt: string
    /** ALLEEN-LEZEN. Reactietype om te tellen. **/
    reacts?: Record<string, number>
    /** ALLEEN-LEZEN **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Waar het media-item naartoe linkt wanneer erop geklikt wordt. **/
    linkUrl?: string
    /** Eén entry per weergave. De widget kiest de beste. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** De linktekst, zoals "Meld je nu aan". **/
    text?: string
    /** Een kop die wordt getoond bij de link. **/
    title?: string
    /** Een beschrijving die wordt getoond bij de link. **/
    description?: string
    url?: string
}
[inline-code-end]

Notes:

- Sommige van deze velden zijn gemarkeerd als `ALLEEN-LEZEN` - deze worden door de API geretourneerd maar kunnen niet worden ingesteld.
- De opmerkingen op een bericht zijn reguliere opmerkingen waarvan de `urlId` `post:` is, gevolgd door de bericht `_id`. Gebruik die waarde met de Comment API om opmerkingen op een bericht te lezen of te maken.