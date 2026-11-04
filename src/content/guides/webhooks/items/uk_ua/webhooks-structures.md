Єдина структура, що надсилається через вебхуки, — це об'єкт WebhookComment, описаний нижче на TypeScript.

#### Структура об'єкта WebhookComment

##### Структура події "Create"
Тіло запиту події "create" — це об'єкт WebhookComment.

##### Структура події "Update"
Тіло запиту події "update" — це об'єкт WebhookComment.

##### Структура події "Delete"
Тіло запиту події "delete" — це об'єкт WebhookComment.

    Зміни станом на 14 листопада 2023 року
    Раніше тіло запиту події "delete" містило лише id коментаря. Тепер воно містить повний коментар на момент видалення.


Кожен ключ завжди присутній у тілі. Коли коментар не має значення для певного поля, тіло містить `null`
(або `false` для логічних значень та `[]` для списків), тому форма доставки ніколи не відрізняється від одного коментаря до іншого.

[inline-code-attrs-start title = 'Об\'єкт WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Ідентифікатор коментаря. **/
    id: string
    /** Ідентифікатор або URL, що ідентифікує гілку коментарів. Нормалізовано. **/
    urlId: string
    /** URL, що вказує на місце, де було залишено коментар. **/
    url: string | null
    /** Ідентифікатор користувача, який залишив коментар. Для SSO — з префіксом tenant id. **/
    userId: string | null
    /** Email користувача, який залишив коментар. **/
    commenterEmail: string | null
    /** Ім'я користувача, що відображається у віджеті коментарів. Для SSO може бути displayName. **/
    commenterName: string
    /** Необроблений текст коментаря. **/
    comment: string
    /** Текст коментаря після обробки. **/
    commentHTML: string
    /** Зовнішній ідентифікатор коментаря. **/
    externalId: string | null
    /** Ідентифікатор батьківського коментаря. **/
    parentId: string | null
    /** UTC-дата, коли було залишено коментар. **/
    date: UTC_ISO_DateString
    /** Сумарна карма (up - down) голосів. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True, якщо користувач був авторизований під час коментування, або верифікував коментар, або верифікував свою сесію, коли коментар було залишено. **/
    verified: boolean
    /** UTC-дата верифікації коментаря. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Чи позначив модератор коментар як перевірений. **/
    reviewed: boolean
    /** Розташування або base64-кодування аватара. Base64 — лише якщо таке значення було передано через SSO. **/
    avatarSrc: string | null
    /** Чи було коментар вручну або автоматично позначено як спам? **/
    isSpam: boolean
    /** Чи було коментар автоматично позначено як спам? **/
    aiDeterminedSpam: boolean
    /** Чи є в коментарі зображення? **/
    hasImages: boolean
    /** Номер сторінки, на якій знаходиться коментар, для сортування "Most Relevant". **/
    pageNumber: number | null
    /** Номер сторінки, на якій знаходиться коментар, для сортування "Oldest First". **/
    pageNumberOF: number | null
    /** Номер сторінки, на якій знаходиться коментар, для сортування "Newest First". **/
    pageNumberNF: number | null
    /** Чи було коментар схвалено автоматично чи вручну? **/
    approved: boolean
    /** Код локалі (формат: en_us) користувача на момент написання коментаря. **/
    locale: string | null
    /** @mentions, написані в коментарі, які було успішно розпізнано. Порожньо, якщо їх немає. **/
    mentions: CommentUserMention[]
    /** Домен, з якого надійшов коментар. **/
    domain: string | null
    /** Ідентифікатори груп модерації, пов'язані з цим коментарем. Порожньо, якщо їх немає. **/
    moderationGroupIds: string[]
    /** Значки, що відображаються поруч з ім'ям коментатора в цьому коментарі, станом на момент створення події. Порожньо, якщо їх немає. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Коли користувачів згадують у коментарі, інформація зберігається у списку під назвою `mentions`. Кожен об'єкт у цьому списку
має таку структуру.

[inline-code-attrs-start title = 'Об\'єкт згадок вебхука'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Ідентифікатор користувача. Для SSO-користувачів він містить префікс з вашим tenant id. **/
    id: string
    /** Кінцевий текст тега @mention, включно з символом @. **/
    tag: string
    /** Оригінальний текст тега @mention, включно з символом @. **/
    rawTag: string
    /** Тип згаданого користувача. user = акаунт FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Якщо користувач відмовився від сповіщень, це значення все одно буде true. **/
    sent: boolean
}
[inline-code-end]

Кожен елемент списку `badges` має таку структуру. Типи значків дивіться в [посібнику зі значками](/guide-badges.html).

[inline-code-attrs-start title = 'Об\'єкт значка вебхука'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** Ідентифікатор значка, наданого цьому користувачу. **/
    id: string
    /** Тип значка. **/
    type: number
    /** Опис значка, що відображається при наведенні курсора. **/
    description: string
    /** Текст, що відображається на значку. **/
    displayLabel?: string | null
    /** Зображення значка, якщо воно використовується. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Додатковий CSS-клас, застосований до значка. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP-методи

Ви можете налаштувати HTTP-метод для кожного типу події вебхука в адмін-панелі:

- **Подія Create**: POST або PUT (за замовчуванням: PUT)
- **Подія Update**: POST або PUT (за замовчуванням: PUT)
- **Подія Delete**: DELETE, POST або PUT (за замовчуванням: DELETE)

Оскільки всі запити містять ID, операції Create та Update є ідемпотентними за замовчуванням (PUT). Повторення того самого запиту Create або Update не повинно створювати дублікати об'єктів на вашому боці.

#### Заголовки запиту

Кожен запит вебхука містить такі заголовки:

| Заголовок | Опис |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ваш API Secret |
| `X-FastComments-Timestamp` | Unix-мітка часу (у секундах), коли запит було підписано |
| `X-FastComments-Signature` | Підпис HMAC-SHA256 (`sha256=<hex>`) |

Інформацію про перевірку підпису HMAC дивіться в розділі [Безпека та API-токени](/guide-webhooks.html#webhooks-api-tokens).