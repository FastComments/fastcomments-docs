A `FeedPost` object represents a post in a FastComments feed. A feed is a stream of posts with their own comment
threads, rendered by the Feed widget. Every post has an author, optional rich content, media, and links, and can be
tagged so that a feed can be filtered.

The structure for the `FeedPost` object is as follows:

[inline-code-attrs-start title = 'FeedPost Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Notes:

- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.
- The comments on a post are regular comments whose `urlId` is `post:` followed by the post `_id`. Use that value with the Comment API to read or create comments on a post.
