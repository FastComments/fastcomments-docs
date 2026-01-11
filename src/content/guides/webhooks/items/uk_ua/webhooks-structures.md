Єдина структура, яка надсилається через вебхуки — це об'єкт WebhookComment, описаний нижче на TypeScript.

#### Структура об'єкта WebhookComment

##### The "Create" Event Structure
Тіло запиту для події "create" — це об'єкт WebhookComment.

##### The "Update" Event Structure
Тіло запиту для події "update" — це об'єкт WebhookComment.

##### The "Delete" Event Structure
Тіло запиту для події "delete" — це об'єкт WebhookComment.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Структура WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Ідентифікатор коментаря. **/
    id: string
    /** id або URL, що ідентифікує тред коментаря. Нормалізовано. **/
    urlId: string
    /** URL, що вказує, де був залишений коментар. **/
    url?: string
    /** id користувача, який залишив коментар. Якщо SSO, з префіксом tenant id. **/
    userId?: string
    /** Електронна пошта користувача, що залишив коментар. **/
    commenterEmail?: string
    /** Ім'я користувача, що відображається у віджеті коментарів. Для SSO може бути displayName. **/
    commenterName: string
    /** Необроблений текст коментаря. **/
    comment: string
    /** Текст коментаря після парсингу. **/
    commentHTML: string
    /** Зовнішній id коментаря. **/
    externalId?: string
    /** id батьківського коментаря. **/
    parentId?: string | null
    /** Дата в UTC, коли був залишений коментар. **/
    date: UTC_ISO_DateString
    /** Сумарна карма голосів (up - down). **/
    votes: number
    votesUp: number
    votesDown: number
    /** True, якщо користувач був увійдений під час коментування, або якщо коментар було верифіковано ним, або якщо його сесія була верифікована на момент залишення коментаря. **/
    verified: boolean
    /** Дата, коли коментар було верифіковано. **/
    verifiedDate?: number
    /** Якщо модератор позначив коментар як переглянутий. **/
    reviewed: boolean
    /** Місцезнаходження або base64-кодування аватарки. Буде у форматі base64 тільки якщо таке значення було передано з SSO. **/
    avatarSrc?: string
    /** Чи був коментар позначений як спам вручну або автоматично? **/
    isSpam: boolean
    /** Чи був коментар автоматично позначений як спам? **/
    aiDeterminedSpam: boolean
    /** Чи містить коментар зображення? **/
    hasImages: boolean
    /** Номер сторінки, на якій знаходиться коментар для сортування "Most Relevant". **/
    pageNumber: number
    /** Номер сторінки для сортування "Oldest First". **/
    pageNumberOF: number
    /** Номер сторінки для сортування "Newest First". **/
    pageNumberNF: number
    /** Чи був коментар схвалений автоматично чи вручну? **/
    approved: boolean
    /** Код локалі (формат: en_us) користувача на момент написання коментаря. **/
    locale: string
    /** @mentions, написані в коментарі, які були успішно розпарсені. **/
    mentions?: CommentUserMention[]
    /** Домен, з якого походить коментар. **/
    domain?: string
    /** Опціональний список id груп модерації, пов'язаних з цим коментарем. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Коли користувачів відмічають у коментарі, інформація зберігається в списку з назвою `mentions`. Кожен об'єкт у цьому списку має таку структуру.

[inline-code-attrs-start title = 'Структура mentions Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** id користувача. Для SSO-користувачів матиме префікс вашого tenant id. **/
    id: string
    /** Остаточний текст @mention тега, включаючи символ @. **/
    tag: string
    /** Початковий текст @mention тега, включаючи символ @. **/
    rawTag: string
    /** Тип тегнутого користувача. user = обліковий запис FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Якщо користувач відмовився від повідомлень, це все одно буде встановлено в true. **/
    sent: boolean
}
[inline-code-end]

#### Використовувані HTTP методи

**Create та Update обидва використовують HTTP PUT, а не POST!**

Оскільки всі наші запити містять id, повторення того самого запиту Create або Update не повинно створювати нові об'єкти на вашому боці.

Це означає, що ці виклики є ідемпотентними і повинні бути подіями PUT відповідно до специфікації HTTP.