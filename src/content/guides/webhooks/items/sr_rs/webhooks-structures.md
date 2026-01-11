Једина структура која се шаље преко webhooks је објекат WebhookComment, описан у TypeScript-у испод.

#### Структура објекта WebhookComment

##### Структура догађаја "create"
Тело захтева за догађај "create" је објекат WebhookComment.

##### Структура догађаја "update"
Тело захтева за догађај "update" је објекат WebhookComment.

##### Структура догађаја "delete"
Тело захтева за догађај "delete" је објекат WebhookComment.

    Change as of 14. новембра 2023.
    Раније тело захтева за догађај "delete" садржало је само comment id. Сада садржи цео коментар у тренутку брисања.


[inline-code-attrs-start title = 'Објекат WebhookComment'; type = 'typescript'; inline-code-attrs-end]
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

Када су корисници означени у коментару, информација се чува у листи названој `mentions`. Сваки објекат у тој листи има следећу структуру.

[inline-code-attrs-start title = 'Објекат помена Webhook'; type = 'typescript'; inline-code-attrs-end]
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

#### HTTP методе које се користе

**Create и Update оба користе HTTP PUT, а не POST!**

Пошто сви наши захтеви садрже ID, поновно слање истог Create или Update захтева не би требало да креира нове објекте на вашој страни.

Ово значи да су ове позиве идемпотентне и да би требало да буду PUT догађаји у складу са HTTP спецификацијом.