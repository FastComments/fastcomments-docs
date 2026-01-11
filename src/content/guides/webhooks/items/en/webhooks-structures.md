The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### The WebhookComment Object Structure

##### The "Create" Event Structure
The "create" event request body is a WebhookComment object.

##### The "Update" Event Structure
The "update" event request body is a WebhookComment object.

##### The "Delete" Event Structure
The "delete" event request body is a WebhookComment object.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'The WebhookComment Object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    url?: string
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    userId?: string
    /** The email of the user left the comment. **/
    commenterEmail?: string
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    commenterName: string
    /** Raw comment text. **/
    comment: string
    /** Comment text after parsing. **/
    commentHTML: string
    /** Comment external id. **/
    externalId?: string
    /** The id of the parent comment. **/
    parentId?: string | null
    /** The UTC date when the comment was left. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    verified: boolean
    /** Date when the comment was verified. **/
    verifiedDate?: number
    /** If a moderator marked the comment reviewed. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    avatarSrc?: string
    /** Was the comment manually or automatically marked as spam? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    pageNumber: number
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    pageNumberOF: number
    /** The page number the comment is on for the "Newest First" sort direction. **/
    pageNumberNF: number
    /** Was the comment approved automatically or manually? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    locale: string
    /** The @mentions written in the comment that were successfully parsed. **/
    mentions?: CommentUserMention[]
    /** The domain the comment is from. **/
    domain?: string
    /** The optional list of moderation group ids associated with this comment. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'The Webhook Mentions Object'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Methods

You can configure the HTTP method for each webhook event type in the admin panel:

- **Create Event**: POST or PUT (default: PUT)
- **Update Event**: POST or PUT (default: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

#### Request Headers

Each webhook request includes the following headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Your API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |

See [Security & API Tokens](/guides/webhooks/webhooks-api-tokens) for information on verifying the HMAC signature.
