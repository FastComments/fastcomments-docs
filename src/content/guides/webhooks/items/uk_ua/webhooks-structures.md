Єдина структура, що надсилається через вебхуки, — це об'єкт WebhookComment, наведений нижче у TypeScript.

#### Структура об'єкта WebhookComment

##### Структура події "Create"
Тіло запиту для події "create" — це об'єкт WebhookComment.

##### Структура події "Update"
Тіло запиту для події "update" — це об'єкт WebhookComment.

##### Структура події "Delete"
Тіло запиту для події "delete" — це об'єкт WebhookComment.

    Зміни станом на 14 листопада 2023 року
    Раніше тіло запиту для події "delete" містило лише id коментаря. Тепер воно містить повний коментар на момент видалення.


[inline-code-attrs-start title = "Об'єкт WebhookComment"; type = 'typescript'; inline-code-attrs-end]
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

Коли користувачів тегають у коментарі, інформація зберігається у списку, названому `mentions`. Кожен об'єкт у цьому списку має таку структуру.

[inline-code-attrs-start title = "Об'єкт згадок вебхука"; type = 'typescript'; inline-code-attrs-end]
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

Ви можете налаштувати HTTP-метод для кожного типу події вебхука в панелі адміністратора:

- **Create Event**: POST або PUT (за замовчуванням: PUT)
- **Update Event**: POST або PUT (за замовчуванням: PUT)
- **Delete Event**: DELETE, POST або PUT (за замовчуванням: DELETE)

Оскільки всі запити містять ID, операції Create та Update за замовчуванням є ідемпотентними (PUT). Повторення одного й того ж запиту Create або Update не повинно створювати дублікати об'єктів на вашій стороні.

#### Request Headers

Кожен запит вебхука включає наступні заголовки:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ваш секрет API |
| `X-FastComments-Timestamp` | Unix-мітка часу (секунди), коли запит було підписано |
| `X-FastComments-Signature` | HMAC-SHA256 підпис (`sha256=<hex>`) |

Див. [Безпека та API-токени](/guides/webhooks/webhooks-api-tokens) для інформації про перевірку підпису HMAC.