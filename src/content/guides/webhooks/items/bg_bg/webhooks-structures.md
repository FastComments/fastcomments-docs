The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### Структура на обекта WebhookComment

##### Структура на събитието "Create"
The "create" event request body is a WebhookComment object.

##### Структура на събитието "Update"
The "update" event request body is a WebhookComment object.

##### Структура на събитието "Delete"
The "delete" event request body is a WebhookComment object.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Промяна от 14 ноември 2023 г.  
Преди тялото на заявката за събитие "delete" съдържаше само идентификатора на коментара. Сега съдържа целия коментар към момента на изтриване.

Всеки ключ винаги присъства в тялото. Когато коментарът няма стойност за дадено поле, тялото съдържа `null`  
(или `false` за булеви стойности и `[]` за списъци), така че формата на доставката никога не се променя от един коментар към друг.

[inline-code-attrs-start title = 'Обектът WebhookComment'; type = 'typescript'; inline-code-attrs-end]
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

Когато потребители са споменати в коментар, информацията се съхранява в списък, наречен `mentions`. Всеки обект в този списък  
има следната структура.

[inline-code-attrs-start title = 'Обектът Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
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

#### HTTP методи

Можете да конфигурирате HTTP метода за всеки тип уебкук събитие в администраторския панел:

- **Create Event**: POST или PUT (по подразбиране: PUT)
- **Update Event**: POST или PUT (по подразбиране: PUT)
- **Delete Event**: DELETE, POST или PUT (по подразбиране: DELETE)

Тъй като всички заявки съдържат ID, операциите Create и Update са идемпотентни по подразбиране (PUT). Повтарянето на една и съща заявка за Create или Update не трябва да създава дублирани обекти от ваша страна.

#### Заглавки на заявката

Всяка уебкук заявка включва следните заглавки:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Вашият API Secret |
| `X-FastComments-Timestamp` | Unix времева отметка (секунди), когато заявката е подписана |
| `X-FastComments-Signature` | HMAC-SHA256 подпис (`sha256=<hex>`) |

Вижте [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) за информация относно проверката на HMAC подписа.