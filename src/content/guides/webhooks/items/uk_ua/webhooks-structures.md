Єдина структура, яку надсилають через вебхуки — це об'єкт WebhookComment, описаний нижче на TypeScript.

#### Структура об'єкта WebhookComment

##### The "Create" Event Structure
Тіло запиту події "create" є об'єктом WebhookComment.

##### The "Update" Event Structure
Тіло запиту події "update" є об'єктом WebhookComment.

##### The "Delete" Event Structure
Тіло запиту події "delete" є об'єктом WebhookComment.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Об'єкт WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Ідентифікатор коментаря. **/
    id: string
    /** Id або URL, що ідентифікує тред коментарів. Нормалізований. **/
    urlId: string
    /** URL, який вказує, де залишено коментар. **/
    url?: string
    /** Id користувача, який залишив коментар. Якщо SSO, з префіксом id орендаря. **/
    userId?: string
    /** Email користувача, який залишив коментар. **/
    commenterEmail?: string
    /** Ім'я користувача, що відображається у віджеті коментарів. Для SSO може бути displayName. **/
    commenterName: string
    /** Сирий текст коментаря. **/
    comment: string
    /** Текст коментаря після парсингу. **/
    commentHTML: string
    /** Зовнішній id коментаря. **/
    externalId?: string
    /** Id батьківського коментаря. **/
    parentId?: string | null
    /** Дата в UTC, коли було залишено коментар. **/
    date: UTC_ISO_DateString
    /** Комбінована карма голосів (up - down). **/
    votes: number
    votesUp: number
    votesDown: number
    /** True, якщо користувач був увійшов у систему під час коментування, або якщо коментар підтверджено, або якщо вони верифікували свою сесію в момент залишення коментаря. **/
    verified: boolean
    /** Дата, коли коментар було підтверджено. **/
    verifiedDate?: number
    /** Чи позначив модератор коментар як переглянутий. **/
    reviewed: boolean
    /** Розташування або base64-код аватару. Буде в base64 лише якщо таке значення було передано з SSO. **/
    avatarSrc?: string
    /** Чи було коментар позначено як спам вручну або автоматично? **/
    isSpam: boolean
    /** Чи було коментар автоматично позначено як спам? **/
    aiDeterminedSpam: boolean
    /** Чи містить коментар зображення? **/
    hasImages: boolean
    /** Номер сторінки, на якій знаходиться коментар для сортування «Найбільш релевантні». **/
    pageNumber: number
    /** Номер сторінки, на якій знаходиться коментар для сортування «Спочатку найстаріші». **/
    pageNumberOF: number
    /** Номер сторінки, на якій знаходиться коментар для сортування «Спочатку найновіші». **/
    pageNumberNF: number
    /** Чи було коментар схвалено автоматично чи вручну? **/
    approved: boolean
    /** Код локалі (формат: en_us) користувача під час написання коментаря. **/
    locale: string
    /** @mention'и, написані в коментарі, які було успішно розпарсено. **/
    mentions?: CommentUserMention[]
    /** Домен, звідки походить коментар. **/
    domain?: string
    /** Опційний список id груп модерації, пов'язаних з цим коментарем. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Об\'єкт згадок вебхука'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id користувача. Для SSO-користувачів тут буде префікс з id орендаря. **/
    id: string
    /** Остаточний текст @mention тегу, включаючи символ @. **/
    tag: string
    /** Початковий текст @mention тегу, включаючи символ @. **/
    rawTag: string
    /** Який тип користувача було зазначено. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Якщо користувач відмовився від сповіщень, це все одно буде встановлено в true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Methods

Ви можете налаштувати HTTP-метод для кожного типу подій вебхука в адмін-панелі:

- **Create Event**: POST або PUT (за замовчуванням: PUT)
- **Update Event**: POST або PUT (за замовчуванням: PUT)
- **Delete Event**: DELETE, POST, або PUT (за замовчуванням: DELETE)

Оскільки всі запити містять ID, операції Create та Update за замовчуванням ідемпотентні (PUT). Повторне надсилання того ж запиту Create або Update не має створювати дублікати об'єктів у вас.

#### Request Headers

Кожний запит вебхука містить такі заголовки:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ваш API Secret |
| `X-FastComments-Timestamp` | Unix-мітка часу (секунди), коли запит було підписано |
| `X-FastComments-Signature` | Підпис HMAC-SHA256 (`sha256=<hex>`) |

See [Безпека та API токени](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.