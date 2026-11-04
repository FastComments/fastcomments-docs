Једина структура која се шаље путем вебхукова је објекат WebhookComment, приказан у TypeScript-у испод.

#### Структура објекта WebhookComment

##### Структура догађаја „Create"
Тело захтева за догађај „create" је објекат WebhookComment.

##### Структура догађаја „Update"
Тело захтева за догађај „update" је објекат WebhookComment.

##### Структура догађаја „Delete"
Тело захтева за догађај „delete" је објекат WebhookComment.

    Промена од 14. новембра 2023.
    Раније је тело захтева за догађај „delete" садржало само id коментара. Сада садржи цео коментар у тренутку брисања.


Сваки кључ је увек присутан у телу. Када коментар нема вредност за неко поље, тело носи `null`
(или `false` за логичке вредности и `[]` за листе), тако да облик испоручене поруке никада не варира од коментара до коментара.

[inline-code-attrs-start title = 'Објекат WebhookComment'; type = 'typescript'; inline-code-attrs-end]
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
    /** The badges shown next to the commenter's name on this comment, as of when the event was created. Empty when there are none. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Када су корисници означени у коментару, информација се чува у листи под називом `mentions`. Сваки објекат у тој листи
има следећу структуру.

[inline-code-attrs-start title = 'Објекат Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
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

Сваки унос у листи `badges` има следећу структуру. Погледајте [водич за значке](/guide-badges.html) за типове значки.

[inline-code-attrs-start title = 'Објекат Webhook Badge'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** The id of the badge awarded to this user. **/
    id: string
    /** The badge type. **/
    type: number
    /** The badge description, shown when hovering over the badge. **/
    description: string
    /** The text shown on the badge. **/
    displayLabel?: string | null
    /** The image shown for the badge, if it uses one. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** An extra CSS class applied to the badge. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP методе

Можете конфигурисати HTTP метод за сваки тип вебхук догађаја у админ панелу:

- **Create Event**: POST или PUT (подразумевано: PUT)
- **Update Event**: POST или PUT (подразумевано: PUT)
- **Delete Event**: DELETE, POST или PUT (подразумевано: DELETE)

Пошто сви захтеви садрже ID, операције Create и Update су подразумевано идемпотентне (PUT). Поновљање истог Create или Update захтева не би требало да креира дупле објекте на вашој страни.

#### Заглавља захтева

Сваки вебхук захтев укључује следећа заглавља:

| Заглавље | Опис |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ваша API тајна |
| `X-FastComments-Timestamp` | Unix временски печат (секунде) када је захтев потписан |
| `X-FastComments-Signature` | HMAC-SHA256 потпис (`sha256=<hex>`) |

Погледајте [Безбедност и API токени](/guide-webhooks.html#webhooks-api-tokens) за информације о верификацији HMAC потписа.