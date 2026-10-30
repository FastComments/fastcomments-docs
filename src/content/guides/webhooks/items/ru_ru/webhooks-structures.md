Единственная структура, отправляемая через вебхуки, — объект WebhookComment, описанный ниже на TypeScript.

#### Структура объекта WebhookComment

##### Структура события "Create"
Тело запроса события "create" представляет собой объект WebhookComment.

##### Структура события "Update"
Тело запроса события "update" представляет собой объект WebhookComment.

##### Структура события "Delete"
Тело запроса события "delete" представляет собой объект WebhookComment.

Изменения от 14 ноября 2023 г.  
Ранее тело запроса события "delete" содержало только идентификатор комментария. Теперь оно содержит полный комментарий на момент удаления.

Каждый ключ всегда присутствует в теле. Когда у комментария нет значения для поля, тело содержит `null`  
(или `false` для булевых значений и `[]` для списков), поэтому структура доставки никогда не меняется от комментария к комментарию.

[inline-code-attrs-start title = 'Объект WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Идентификатор комментария. **/
    id: string
    /** Идентификатор или URL, определяющий ветку комментариев. Нормализовано. **/
    urlId: string
    /** URL, указывающий место, где был оставлен комментарий. **/
    url: string | null
    /** Идентификатор пользователя, оставившего комментарий. Если SSO, с префиксом tenant id. **/
    userId: string | null
    /** Электронная почта пользователя, оставившего комментарий. **/
    commenterEmail: string | null
    /** Имя пользователя, отображаемое в виджете комментария. При SSO может быть displayName. **/
    commenterName: string
    /** Необработанный текст комментария. **/
    comment: string
    /** Текст комментария после парсинга. **/
    commentHTML: string
    /** Внешний идентификатор комментария. **/
    externalId: string | null
    /** Идентификатор родительского комментария. **/
    parentId: string | null
    /** Дата в UTC, когда был оставлен комментарий. **/
    date: UTC_ISO_DateString
    /** Суммарный карма (голосов "за" - "против"). **/
    votes: number
    votesUp: number
    votesDown: number
    /** true, если пользователь был залогинен при комментировании, или их комментарий был верифицирован, или они верифицировали сессию при оставлении комментария. **/
    verified: boolean
    /** Дата в UTC, когда комментарий был верифицирован. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Если модератор отметил комментарий как проверенный. **/
    reviewed: boolean
    /** Расположение или base64-кодировка аватара. Будет base64 только если это значение было передано через SSO. **/
    avatarSrc: string | null
    /** Был ли комментарий помечен как спам вручную или автоматически? **/
    isSpam: boolean
    /** Был ли комментарий автоматически помечен как спам? **/
    aiDeterminedSpam: boolean
    /** Есть ли изображения в комментарии? **/
    hasImages: boolean
    /** Номер страницы, на которой находится комментарий при сортировке "Самые релевантные". **/
    pageNumber: number | null
    /** Номер страницы, на которой находится комментарий при сортировке "Старые сначала". **/
    pageNumberOF: number | null
    /** Номер страницы, на которой находится комментарий при сортировке "Новые сначала". **/
    pageNumberNF: number | null
    /** Был ли комментарий одобрен автоматически или вручную? **/
    approved: boolean
    /** Код локали (формат: en_us) пользователя, когда был написан комментарий. **/
    locale: string | null
    /** @упоминания, написанные в комментарии и успешно распарсенные. Пусто, если их нет. **/
    mentions: CommentUserMention[]
    /** Домен, откуда пришел комментарий. **/
    domain: string | null
    /** Идентификаторы групп модерации, связанные с этим комментарием. Пусто, если их нет. **/
    moderationGroupIds: string[]
    /** Значки, отображаемые рядом с именем комментатора в этом комментарии, на момент создания события. Пусто, если их нет. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Когда пользователи упоминаются в комментарии, информация сохраняется в списке под названием `mentions`. Каждый объект в этом списке имеет следующую структуру.

[inline-code-attrs-start title = 'Объект упоминаний Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Идентификатор пользователя. Для SSO‑пользователей будет с префиксом tenant id. **/
    id: string
    /** Окончательный текст тега @mention, включая символ @. **/
    tag: string
    /** Оригинальный текст тега @mention, включая символ @. **/
    rawTag: string
    /** Какой тип пользователя был упомянут. user = аккаунт FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Если пользователь отказался от уведомлений, это всё равно будет true. **/
    sent: boolean
}
[inline-code-end]

Каждый элемент списка `badges` имеет следующую структуру. Смотрите [руководство по значкам](/guide-badges.html) для типов значков.

[inline-code-attrs-start title = 'Объект значка Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** Идентификатор значка, присвоенного этому пользователю. **/
    id: string
    /** Тип значка. **/
    type: number
    /** Описание значка, отображаемое при наведении. **/
    description: string
    /** Текст, отображаемый на значке. **/
    displayLabel?: string | null
    /** Изображение, отображаемое для значка, если оно используется. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Дополнительный CSS‑класс, применяемый к значку. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP-методы

Вы можете настроить HTTP-метод для каждого типа события вебхука в админ-панели:

- **Create Event**: POST или PUT (по умолчанию: PUT)
- **Update Event**: POST или PUT (по умолчанию: PUT)
- **Delete Event**: DELETE, POST или PUT (по умолчанию: DELETE)

Поскольку все запросы содержат ID, операции Create и Update по умолчанию идемпотентны (PUT). Повторение того же запроса Create или Update не должно создавать дублирующие объекты у вас.

#### Заголовки запроса

Каждый запрос вебхука включает следующие заголовки:

| Header | Описание |
|--------|----------|
| `Content-Type` | `application/json` |
| `token` | Ваш API‑секрет |
| `X-FastComments-Timestamp` | Unix‑временная метка (секунды) при подписи запроса |
| `X-FastComments-Signature` | HMAC‑SHA256 подпись (`sha256=<hex>`) |

Смотрите [Безопасность и API‑токены](/guide-webhooks.html#webhooks-api-tokens) для информации о проверке HMAC‑подписи.

---