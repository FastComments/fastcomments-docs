Единственная структура, отправляемая через вебхуки, — это объект WebhookComment, описанный ниже на TypeScript.

#### Структура объекта WebhookComment

##### Структура события "create"
Тело запроса для события "create" — объект WebhookComment.

##### Структура события "update"
Тело запроса для события "update" — объект WebhookComment.

##### Структура события "delete"
Тело запроса для события "delete" — объект WebhookComment.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Объект WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Идентификатор комментария. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** URL, указывающий место, где был оставлен комментарий. **/
    url?: string
    /** Идентификатор пользователя, оставившего комментарий. Для SSO — с префиксом tenant id. **/
    userId?: string
    /** Электронная почта пользователя, оставившего комментарий. **/
    commenterEmail?: string
    /** Имя пользователя, отображаемое в виджете комментариев. Для SSO может быть displayName. **/
    commenterName: string
    /** Исходный текст комментария. **/
    comment: string
    /** Текст комментария после парсинга. **/
    commentHTML: string
    /** Внешний идентификатор комментария. **/
    externalId?: string
    /** Идентификатор родительского комментария. **/
    parentId?: string | null
    /** UTC-дата, когда был оставлен комментарий. **/
    date: UTC_ISO_DateString
    /** Суммарная карма (за - против) голосов. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True если пользователь был залогинен при написании комментария, либо он верифицировал комментарий, либо он подтвердил сессию в момент оставления комментария. **/
    verified: boolean
    /** Дата, когда комментарий был верифицирован. **/
    verifiedDate?: number
    /** Если модератор пометил комментарий как просмотренный. **/
    reviewed: boolean
    /** Расположение или base64-кодирование аватара. Будет в base64 только если такое значение было передано с SSO. **/
    avatarSrc?: string
    /** Был ли комментарий помечен как спам вручную или автоматически? **/
    isSpam: boolean
    /** Был ли комментарий автоматически помечен как спам? **/
    aiDeterminedSpam: boolean
    /** Содержатся ли в комментарии изображения? **/
    hasImages: boolean
    /** Номер страницы, на которой находится комментарий для сортировки "Most Relevant". **/
    pageNumber: number
    /** Номер страницы, на которой находится комментарий для сортировки "Oldest First". **/
    pageNumberOF: number
    /** Номер страницы, на которой находится комментарий для сортировки "Newest First". **/
    pageNumberNF: number
    /** Была ли публикация комментария одобрена автоматически или вручную? **/
    approved: boolean
    /** Код локали (формат: en_us) пользователя в момент написания комментария. **/
    locale: string
    /** @упоминания, написанные в комментарии и успешно распарсенные. **/
    mentions?: CommentUserMention[]
    /** Домен, откуда пришёл комментарий. **/
    domain?: string
    /** Необязательный список идентификаторов групп модерации, связанных с этим комментарием. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Когда пользователей отмечают в комментарии, информация хранится в списке под названием `mentions`. Каждый объект в этом списке имеет следующую структуру.

[inline-code-attrs-start title = 'Объект упоминаний вебхука'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Идентификатор пользователя. Для SSO-пользователей будет с префиксом tenant id. **/
    id: string
    /** Окончательный текст тега @mention, включая символ @. **/
    tag: string
    /** Исходный текст тега @mention, включая символ @. **/
    rawTag: string
    /** Тип отмеченного пользователя. user = аккаунт FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Если пользователь отказался от уведомлений, это всё равно будет установлено в true. **/
    sent: boolean
}
[inline-code-end]

#### Используемые HTTP-методы

**События Create и Update используют HTTP PUT, а не POST!**

Поскольку все наши запросы содержат ID, повторение того же запроса Create или Update не должно создавать новые объекты на вашей стороне.

Это означает, что эти вызовы идемпотентны и согласно спецификации HTTP должны быть событиями PUT.