Единственная структура, отправляемая через вебхуки — это объект WebhookComment, приведённый ниже на TypeScript.

#### Структура объекта WebhookComment

##### The "Create" Event Structure
Тело запроса события "create" — объект WebhookComment.

##### The "Update" Event Structure
Тело запроса события "update" — объект WebhookComment.

##### The "Delete" Event Structure
Тело запроса события "delete" — объект WebhookComment.

    Изменение от 14 ноября 2023 г.
    Ранее тело запроса события "delete" содержало только id комментария. Теперь оно содержит полный комментарий на момент удаления.


[inline-code-attrs-start title = 'Объект WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Идентификатор комментария. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** URL, указывающий, где был оставлен комментарий. **/
    url?: string
    /** id пользователя, который оставил комментарий. Для SSO префикс — tenant id. **/
    userId?: string
    /** Email пользователя, который оставил комментарий. **/
    commenterEmail?: string
    /** Имя пользователя, отображаемое в виджете комментариев. Для SSO может быть displayName. **/
    commenterName: string
    /** Исходный текст комментария. **/
    comment: string
    /** Текст комментария после парсинга. **/
    commentHTML: string
    /** Внешний id комментария. **/
    externalId?: string
    /** id родительского комментария. **/
    parentId?: string | null
    /** Дата в UTC, когда комментарий был оставлен. **/
    date: UTC_ISO_DateString
    /** Суммарная карма голосов (за - против). **/
    votes: number
    votesUp: number
    votesDown: number
    /** True — если пользователь был вошедшим в систему при оставлении комментария, либо если комментарий был верифицирован, либо если сессия была верифицирована при оставлении комментария. **/
    verified: boolean
    /** Дата, когда комментарий был верифицирован. **/
    verifiedDate?: number
    /** Отмечено ли модератором, что комментарий просмотрен. **/
    reviewed: boolean
    /** Местоположение или base64-кодированное изображение аватара. Будет в base64 только если такое значение было передано с SSO. **/
    avatarSrc?: string
    /** Был ли комментарий помечен как спам вручную или автоматически? **/
    isSpam: boolean
    /** Был ли комментарий автоматически помечен как спам? **/
    aiDeterminedSpam: boolean
    /** Содержит ли комментарий изображения? **/
    hasImages: boolean
    /** Номер страницы, на которой находится комментарий при сортировке "Most Relevant". **/
    pageNumber: number
    /** Номер страницы, на которой находится комментарий при сортировке "Oldest First". **/
    pageNumberOF: number
    /** Номер страницы, на которой находится комментарий при сортировке "Newest First". **/
    pageNumberNF: number
    /** Был ли комментарий одобрен автоматически или вручную? **/
    approved: boolean
    /** Код локали (формат: en_us) пользователя в момент написания комментария. **/
    locale: string
    /** @упоминания, указанные в комментарии и успешно распарсенные. **/
    mentions?: CommentUserMention[]
    /** Домен, с которого пришёл комментарий. **/
    domain?: string
    /** Необязательный список id групп модерации, связанных с этим комментарием. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Объект упоминаний Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** Финальный текст тега @mention, включая символ @. **/
    tag: string
    /** Исходный текст тега @mention, включая символ @. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Даже если пользователь отказался от уведомлений, это поле всё равно будет установлено в true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Methods

Вы можете настроить HTTP-метод для каждого типа события вебхука в панели администратора:

- **Create Event**: POST or PUT (default: PUT)
- **Update Event**: POST or PUT (default: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)

Поскольку все запросы содержат ID, операции Create и Update по умолчанию идемпотентны (PUT). Повторение того же запроса Create или Update не должно создавать дубликаты объектов на вашей стороне.

#### Request Headers

Каждый запрос вебхука содержит следующие заголовки:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Секрет вашего API |
| `X-FastComments-Timestamp` | Unix-временная метка (в секундах), когда запрос был подписан |
| `X-FastComments-Signature` | Подпись HMAC-SHA256 (`sha256=<hex>`) |

См. [Безопасность и API-токены](/guides/webhooks/webhooks-api-tokens) для информации о проверке HMAC-подписи.

---