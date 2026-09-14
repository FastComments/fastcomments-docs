A `FeedPost` object represents a post in a FastComments feed. A feed is a stream of posts with their own comment threads, rendered by the Feed widget. Every post has an author, optional rich content, media, and links, and can be tagged so that a feed can be filtered.

The structure for the `FeedPost` object is as follows:

[inline-code-attrs-start title = 'Struktura FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** Id użytkownika FastComments lub SSO, który jest autorem posta. **/
    fromUserId?: string
    /** Wypełniane z danych użytkownika, gdy nie jest ustawione. **/
    fromUserDisplayName?: string | null
    /** READONLY. Wypełniane z danych użytkownika. **/
    fromUserAvatar?: string | null
    /** Używane do filtrowania kanału. **/
    tags?: string[]
    /** Waga sortowania w kanale. Wyższe wartości sortują jako pierwsze. **/
    weight?: number
    /** Dowolne pary klucz/wartość do własnego użytku. **/
    meta?: Record<string, string>
    /** Zsanityzowany HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Reaction type to count. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Gdzie odnośnik elementu multimedialnego prowadzi po kliknięciu. **/
    linkUrl?: string
    /** Jedna pozycja na każdą wersję. Widget wybiera najlepsze dopasowanie. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Tekst linku, np. “Zarejestruj się teraz”. **/
    text?: string
    /** Nagłówek wyświetlany razem z linkiem. **/
    title?: string
    /** Opis wyświetlany razem z linkiem. **/
    description?: string
    url?: string
}
[inline-code-end]

Notes:

- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.
- The comments on a post are regular comments whose `urlId` is `post:` followed by the post `_id`. Use that value with the Comment API to read or create comments on a post.