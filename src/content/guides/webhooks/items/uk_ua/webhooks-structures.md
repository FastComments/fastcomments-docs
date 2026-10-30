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

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

[inline-code-attrs-start title = 'Об\'єкт WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Ідентифікатор коментаря. **/
    id: string
    /** Ідентифікатор або URL, що ідентифікує гілку коментаря. Нормалізовано. **/
    urlId: string
    /** URL, який вказує, де був залишений коментар. **/
    url: string | null
    /** Ідентифікатор користувача, який залишив коментар. Якщо SSO, префікс з ідентифікатором орендаря. **/
    userId: string | null
    /** Електронна пошта користувача, який залишив коментар. **/
    commenterEmail: string | null
    /** Ім'я користувача, що відображається у віджеті коментаря. При SSO може бути displayName. **/
    commenterName: string
    /** Необроблений текст коментаря. **/
    comment: string
    /** Текст коментаря після парсингу. **/
    commentHTML: string
    /** Зовнішній ідентифікатор коментаря. **/
    externalId: string | null
    /** Ідентифікатор батьківського коментаря. **/
    parentId: string | null
    /** Дата у форматі UTC, коли був залишений коментар. **/
    date: UTC_ISO_DateString
    /** Комбінована карма (up - down) голосів. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True, якщо користувач був увійшов у систему під час коментування, або його коментар був верифікований, або якщо він верифікував свою сесію під час залишення коментаря. **/
    verified: boolean
    /** Дата у форматі UTC, коли коментар був верифікований. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Якщо модератор позначив коментар як переглянутий. **/
    reviewed: boolean
    /** Розташування або base64-кодування аватара. Буде у форматі base64 лише якщо це значення було передано через SSO. **/
    avatarSrc: string | null
    /** Чи був коментар позначений як спам вручну або автоматично? **/
    isSpam: boolean
    /** Чи був коментар автоматично позначений як спам? **/
    aiDeterminedSpam: boolean
    /** Чи є зображення в коментарі? **/
    hasImages: boolean
    /** Номер сторінки, на якій знаходиться коментар, для сортування "Most Relevant". **/
    pageNumber: number | null
    /** Номер сторінки, на якій знаходиться коментар, для сортування "Oldest First". **/
    pageNumberOF: number | null
    /** Номер сторінки, на якій знаходиться коментар, для сортування "Newest First". **/
    pageNumberNF: number | null
    /** Чи був коментар схвалений автоматично або вручну? **/
    approved: boolean
    /** Код локалі (формат: en_us) користувача під час написання коментаря. **/
    locale: string | null
    /** Згадки @, написані в коментарі, які успішно розпарсені. Порожньо, якщо їх немає. **/
    mentions: CommentUserMention[]
    /** Домен, з якого надійшов коментар. **/
    domain: string | null
    /** Ідентифікатори груп модерації, пов'язані з цим коментарем. Порожньо, якщо їх немає. **/
    moderationGroupIds: string[]
    /** Значки, що відображаються поруч з ім'ям коментатора в цьому коментарі, на момент створення події. Порожньо, якщо їх немає. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list has the following structure.

[inline-code-attrs-start title = 'Об\'єкт згадок Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Ідентифікатор користувача. Для користувачів SSO це буде мати префікс вашого ідентифікатора орендаря. **/
    id: string
    /** Остаточний текст тегу @mention, включаючи символ @. **/
    tag: string
    /** Оригінальний текст тегу @mention, включаючи символ @. **/
    rawTag: string
    /** Тип користувача, який був згаданий. user = обліковий запис FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Якщо користувач відмовився від сповіщень, це все одно буде встановлено в true. **/
    sent: boolean
}
[inline-code-end]

Each entry in the `badges` list has the following structure. See the [Badges guide](/guide-badges.html) for the badge types.

[inline-code-attrs-start title = 'Об\'єкт значка Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** Ідентифікатор значка, наданого цьому користувачу. **/
    id: string
    /** Тип значка. **/
    type: number
    /** Опис значка, що відображається при наведенні курсору. **/
    description: string
    /** Текст, що відображається на значку. **/
    displayLabel?: string | null
    /** Зображення, що відображається для значка, якщо воно використовується. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Додатковий CSS-клас, застосований до значка. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP-методи

You can configure the HTTP method for each webhook event type in the admin panel:

- **Подія створення**: POST або PUT (за замовчуванням: PUT)
- **Подія оновлення**: POST або PUT (за замовчуванням: PUT)
- **Подія видалення**: DELETE, POST або PUT (за замовчуванням: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

#### Заголовки запиту

Each webhook request includes the following headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ваш секрет API |
| `X-FastComments-Timestamp` | Unix-часова мітка (секунди) коли запит був підписаний |
| `X-FastComments-Signature` | HMAC-SHA256 підпис (`sha256=<hex>`) |

See [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.