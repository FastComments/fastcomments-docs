Единствената структура, изпращана чрез уебхукове, е обектът WebhookComment, описан в TypeScript по-долу.

#### Структура на обекта WebhookComment

##### Структура за събитие "Create"
Тялото на заявката за събитие "create" е обект WebhookComment.

##### Структура за събитие "Update"
Тялото на заявката за събитие "update" е обект WebhookComment.

##### Структура за събитие "Delete"
Тялото на заявката за събитие "delete" е обект WebhookComment.

    Промяна от 14 ноември 2023 г.
    Преди това тялото на заявката за събитие "delete" съдържаше само id на коментара. Сега съдържа целия коментар в момента на изтриване.


[inline-code-attrs-start title = 'Обектът WebhookComment'; type = 'typescript'; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Обектът за споменавания в Webhook'; type = 'typescript'; inline-code-attrs-end]
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

Можете да конфигурирате HTTP метода за всеки тип уебхук събитие в администраторския панел:

- **Create Event**: POST or PUT (default: PUT)
- **Update Event**: POST or PUT (default: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)

Тъй като всички заявки съдържат ID, операциите Create и Update са идемпотентни по подразбиране (PUT). Повтарянето на една и съща Create или Update заявка не трябва да създава дублирани обекти от ваша страна.

#### Request Headers

Всяка уебхук заявка включва следните хедъри:

| Заглавка | Описание |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Your API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |

Вижте [Сигурност и API токени](/guide-webhooks.html#webhooks-api-tokens) за информация относно проверката на HMAC подписа.