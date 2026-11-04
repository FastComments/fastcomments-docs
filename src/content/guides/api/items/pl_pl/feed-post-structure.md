Obiekt `FeedPost` reprezentuje wpis w feedzie FastComments. Feed to strumień wpisów z własnymi wątkami komentarzy, renderowany przez widżet Feed. Każdy wpis ma autora, opcjonalną treść sformatowaną, media i linki, a może być też otagowany, aby feed mógł być filtrowany.

Struktura obiektu `FeedPost` wygląda następująco:

[inline-code-attrs-start title = 'Struktura FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** The id of the FastComments or SSO user that authored the post. **/
    fromUserId?: string
    /** Filled from the user when not set. **/
    fromUserDisplayName?: string | null
    /** READONLY. Filled from the user. **/
    fromUserAvatar?: string | null
    /** Used to filter a feed. **/
    tags?: string[]
    /** Sort weight within a feed. Higher values sort first. **/
    weight?: number
    /** Free-form key/value pairs for your own use. **/
    meta?: Record<string, string>
    /** Sanitized HTML. **/
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
    /** Where the media item links to when clicked. **/
    linkUrl?: string
    /** One entry per rendition. The widget picks the best fit. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** The link text, like "Sign up now". **/
    text?: string
    /** A heading shown with the link. **/
    title?: string
    /** A description shown with the link. **/
    description?: string
    url?: string
}
[inline-code-end]

Uwagi:

- Niektóre z tych pól są oznaczone jako `READONLY` - są zwracane przez API, ale nie można ich ustawiać.
- Komentarze pod wpisem to zwykłe komentarze, których `urlId` to `post:` po którym następuje `_id` wpisu. Użyj tej wartości z Comment API, aby odczytać lub utworzyć komentarze pod wpisem.