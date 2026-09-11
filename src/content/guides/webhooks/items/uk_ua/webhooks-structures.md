The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### Структура об’єкта WebhookComment

##### Структура події "Create"
The "create" event request body is a WebhookComment object.

##### Структура події "Update"
The "update" event request body is a WebhookComment object.

##### Структура події "Delete"
The "delete" event request body is a WebhookComment object.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Зміна станом на 14 листопада 2023 року  
Раніше тіло запиту події "delete" містило лише ідентифікатор коментаря. Тепер воно містить повний коментар на момент видалення.

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

Кожен ключ завжди присутній у тілі. Якщо у коментаря немає значення для поля, тіло містить `null` (або `false` для булевих значень і `[]` для списків), тому структура доставки ніколи не змінюється від коментаря до коментаря.

[inline-code-attrs-start title = 'Об’єкт WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    url: string | null
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    userId: string | null
    /** The email of the user left the comment. **/
    commenterEmail: string | null
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    commenterName: string
    /** Raw comment text. **/
    comment: string
    /** Comment text after parsing. **/
    commentHTML: string
    /** Comment external id. **/
    externalId: string | null
    /** The id of the parent comment. **/
    parentId: string | null
    /** The UTC date when the comment was left. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    verified: boolean
    /** The UTC date when the comment was verified. **/
    verifiedDate: UTC_ISO_DateString | null
    /** If a moderator marked the comment reviewed. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    avatarSrc: string | null
    /** Was the comment manually or automatically marked as spam? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    pageNumber: number | null
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    pageNumberOF: number | null
    /** The page number the comment is on for the "Newest First" sort direction. **/
    pageNumberNF: number | null
    /** Was the comment approved automatically or manually? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    locale: string | null
    /** The @mentions written in the comment that were successfully parsed. Empty when there are none. **/
    mentions: CommentUserMention[]
    /** The domain the comment is from. **/
    domain: string | null
    /** The moderation group ids associated with this comment. Empty when there are none. **/
    moderationGroupIds: string[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

Коли користувачі згадуються в коментарі, інформація зберігається у списку під назвою `mentions`. Кожен об’єкт у цьому списку має таку структуру.

[inline-code-attrs-start title = 'Об’єкт Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
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

#### HTTP-методи

You can configure the HTTP method for each webhook event type in the admin panel:

- **Create Event**: POST або PUT (за замовчуванням: PUT)
- **Update Event**: POST або PUT (за замовчуванням: PUT)
- **Delete Event**: DELETE, POST або PUT (за замовчуванням: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

Оскільки всі запити містять ID, операції Create та Update за замовчуванням є ідемпотентними (PUT). Повторення одного і того ж запиту Create або Update не повинно створювати дублікати об’єктів у вас.

#### Заголовки запиту

Each webhook request includes the following headers:

| Заголовок | Опис |
|-----------|------|
| `Content-Type` | `application/json` |
| `token` | Ваш секрет API |
| `X-FastComments-Timestamp` | Unix‑таймстамп (секунди) коли запит був підписаний |
| `X-FastComments-Signature` | Підпис HMAC‑SHA256 (`sha256=<hex>`) |

See [Безпека та токени API](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.