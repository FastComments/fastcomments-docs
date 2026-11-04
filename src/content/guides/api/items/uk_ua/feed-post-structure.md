Об'єкт `FeedPost` представляє допис у стрічці FastComments. Стрічка — це потік дописів з власними гілками коментарів, який відображається віджетом Feed. Кожен допис має автора, необов'язковий розширений вміст, медіа та посилання, і може бути позначений тегами, щоб стрічку можна було фільтрувати.

Структура об'єкта `FeedPost` виглядає так:

[inline-code-attrs-start title = 'Структура FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Примітки:

- Деякі з цих полів позначені як `READONLY` — вони повертаються API, але їх не можна встановлювати.
- Коментарі до допису — це звичайні коментарі, чиї `urlId` мають вигляд `post:` з наступним `_id` допису. Використовуйте це значення з Comment API, щоб читати або створювати коментарі до допису.