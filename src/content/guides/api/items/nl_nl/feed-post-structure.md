Een `FeedPost`-object vertegenwoordigt een bericht in een FastComments-feed. Een feed is een stroom van berichten met hun eigen commentaarthreads, gerenderd door de Feed-widget. Elk bericht heeft een auteur, optionele rich content, media en links, en kan worden getagd zodat een feed kan worden gefilterd.

De structuur van het `FeedPost`-object is als volgt:

[inline-code-attrs-start title = 'FeedPost Structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Opmerkingen:

- Sommige van deze velden zijn gemarkeerd als `READONLY` - deze worden door de API geretourneerd maar kunnen niet worden ingesteld.
- De comments op een bericht zijn gewone comments waarvan de `urlId` `post:` is gevolgd door de `_id` van het bericht. Gebruik die waarde met de Comment API om comments op een bericht te lezen of aan te maken.