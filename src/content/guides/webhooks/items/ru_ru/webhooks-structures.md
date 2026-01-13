Единственная структура, отправляемая через вебхуки, — это объект WebhookComment, описанный ниже на TypeScript.

#### Структура объекта WebhookComment

##### Структура события "Create"
Тело запроса для события "create" — это объект WebhookComment.

##### Структура события "Update"
Тело запроса для события "update" — это объект WebhookComment.

##### Структура события "Delete"
Тело запроса для события "delete" — это объект WebhookComment.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Объект WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Идентификатор комментария. **/
    id: string
    /** Идентификатор или URL, идентифицирующий тред комментариев. Нормализовано. **/
    urlId: string
    /** URL, указывающий, где был оставлен комментарий. **/
    url?: string
    /** Идентификатор пользователя, оставившего комментарий. Для SSO — с префиксом tenant id. **/
    userId?: string
    /** Email пользователя, оставившего комментарий. **/
    commenterEmail?: string
    /** Имя пользователя, отображаемое в виджете комментариев. Для SSO может быть displayName. **/
    commenterName: string
    /** Исходный текст комментария. **/
    comment: string
    /** Текст комментария после парсинга. **/
    commentHTML: string
    /** Внешний id комментария. **/
    externalId?: string
    /** Идентификатор родительского комментария. **/
    parentId?: string | null
    /** UTC-дата, когда был оставлен комментарий. **/
    date: UTC_ISO_DateString
    /** Комбинированная карма (за - против) голосов. **/
    votes: number
    votesUp: number
    votesDown: number
    /** true, если пользователь был залогинен при написании комментария, либо комментарий был верифицирован, либо сессия была подтверждена при оставлении комментария. **/
    verified: boolean
    /** Дата верификации комментария. **/
    verifiedDate?: number
    /** Если модератор пометил комментарий как просмотренный. **/
    reviewed: boolean
    /** Местоположение или base64-кодирование аватара. Будет в base64 только если такое значение было передано через SSO. **/
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
    /** @упоминания в комментарии, которые были успешно распознаны. **/
    mentions?: CommentUserMention[]
    /** Домен, откуда пришел комментарий. **/
    domain?: string
    /** Необязательный список id групп модерации, связанных с этим комментарием. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Когда пользователи отмечены в комментарии, информация хранится в списке с именем `mentions`. Каждый объект в этом списке имеет следующую структуру.

[inline-code-attrs-start title = 'Объект упоминаний Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Идентификатор пользователя. Для SSO-пользователей будет с префиксом tenant id. **/
    id: string
    /** Итоговый текст тега @mention, включая символ @. **/
    tag: string
    /** Оригинальный текст тега @mention, включая символ @. **/
    rawTag: string
    /** Тип отмеченного пользователя. user = аккаунт FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Даже если пользователь отказывается от уведомлений, это все равно будет установлено в true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP-методы

Вы можете настроить HTTP-метод для каждого типа события вебхука в панели администратора:

- **Create Event**: POST или PUT (по умолчанию: PUT)
- **Update Event**: POST или PUT (по умолчанию: PUT)
- **Delete Event**: DELETE, POST, или PUT (по умолчанию: DELETE)

Поскольку все запросы содержат ID, операции Create и Update по умолчанию идемпотентны (PUT). Повторная отправка того же запроса Create или Update не должна создавать дубликаты объектов на вашей стороне.

#### Заголовки запроса

Каждый запрос вебхука содержит следующие заголовки:

| Заголовок | Описание |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ваш секрет API |
| `X-FastComments-Timestamp` | UNIX-временная метка (в секундах), когда запрос был подписан |
| `X-FastComments-Signature` | Подпись HMAC-SHA256 (`sha256=<hex>`) |

См. [Security & API Tokens](/guides/webhooks/webhooks-api-tokens) для информации о проверке подписи HMAC.

---