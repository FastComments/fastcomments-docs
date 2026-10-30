Единствената структура, изпращана чрез уебкуки, е обектът WebhookComment, описан в TypeScript по-долу.

#### Структура на обекта WebhookComment

##### Структура на събитието "Create"
Събитието "create" заявка съдържа обект WebhookComment.

##### Структура на събитието "Update"
Събитието "update" заявка съдържа обект WebhookComment.

##### Структура на събитието "Delete"
Събитието "delete" заявка съдържа обект WebhookComment.

Промяна от 14 ноември 2023 г. Преди тялото на заявката за събитието "delete" съдържаше само идентификатора на коментара. Сега съдържа целия коментар към момента на изтриване.

Всеки ключ винаги присъства в тялото. Когато коментарът няма стойност за дадено поле, тялото съдържа `null` (или `false` за булеви стойности и `[]` за списъци), така че формата на доставката никога не варира от един коментар към друг.

[inline-code-attrs-start title = 'Обектът WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Идентификаторът на коментара. **/
    id: string
    /** Идентификаторът или URL, който идентифицира нишката на коментара. Нормализиран. **/
    urlId: string
    /** URL‑то, което указва къде е оставен коментарът. **/
    url: string | null
    /** Идентификаторът на потребителя, който е оставил коментара. При SSO е с префикс на идентификатора на наемателя. **/
    userId: string | null
    /** Имейлът на потребителя, който е оставил коментара. **/
    commenterEmail: string | null
    /** Името на потребителя, което се показва в уиджета за коментари. При SSO може да бъде displayName. **/
    commenterName: string
    /** Необработен текст на коментара. **/
    comment: string
    /** Текст на коментара след парсиране. **/
    commentHTML: string
    /** Външен идентификатор на коментара. **/
    externalId: string | null
    /** Идентификаторът на родителския коментар. **/
    parentId: string | null
    /** UTC датата, когато е оставен коментарът. **/
    date: UTC_ISO_DateString
    /** Комбинирано карма (гласове нагоре - надолу). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Истина, ако потребителят е бил влязъл, когато е коментирал, или е верифицирал коментара, или е верифицирал сесията си, когато коментарът е оставен. **/
    verified: boolean
    /** UTC датата, когато коментарът е верифициран. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Ако модератор е маркирал коментара като прегледан. **/
    reviewed: boolean
    /** Местоположението или base64 кодирането на аватара. Ще бъде base64 само ако това е стойността, предадена с SSO. **/
    avatarSrc: string | null
    /** Беше ли коментарът маркиран като спам ръчно или автоматично? **/
    isSpam: boolean
    /** Беше ли коментарът автоматично маркиран като спам? **/
    aiDeterminedSpam: boolean
    /** Има ли изображения в коментара? **/
    hasImages: boolean
    /** Номерът на страницата, на която се намира коментарът за сортиране "Most Relevant". **/
    pageNumber: number | null
    /** Номерът на страницата, на която се намира коментарът за сортиране "Oldest First". **/
    pageNumberOF: number | null
    /** Номерът на страницата, на която се намира коментарът за сортиране "Newest First". **/
    pageNumberNF: number | null
    /** Беше ли коментарът одобрен автоматично или ръчно? **/
    approved: boolean
    /** Кодът на локала (формат: en_us) на потребителя, когато е написан коментарът. **/
    locale: string | null
    /** Отбелязаните @упомени в коментара, които са успешно парсирани. Празно, ако няма такива. **/
    mentions: CommentUserMention[]
    /** Домейнът, от който е коментарът. **/
    domain: string | null
    /** Идентификаторите на модерационните групи, свързани с този коментар. Празно, ако няма такива. **/
    moderationGroupIds: string[]
    /** Значките, показвани до името на коментатора в този коментар, към момента на създаване на събитието. Празно, ако няма такива. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Когато потребители са отбелязани в коментар, информацията се съхранява в списък, наречен `mentions`. Всеки обект в този списък има следната структура.

[inline-code-attrs-start title = 'Обектът Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Идентификаторът на потребителя. За SSO потребители това ще има вашия tenant id като префикс. **/
    id: string
    /** Крайната @упомената текстова стойност, включително символа @. **/
    tag: string
    /** Оригиналният @упоменат текст, включително символа @. **/
    rawTag: string
    /** Какъв тип потребител е отбелязан. user = акаунт в FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ако потребителят се откаже от известия, това все пак ще бъде зададено на true. **/
    sent: boolean
}
[inline-code-end]

Всеки елемент от списъка `badges` има следната структура. Вижте [Badges guide](/guide-badges.html) за типове значки.

[inline-code-attrs-start title = 'Обектът Webhook Badge'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** Идентификаторът на значката, присвоена на този потребител. **/
    id: string
    /** Типът на значката. **/
    type: number
    /** Описанието на значката, показвано при задържане на курсора. **/
    description: string
    /** Текстът, показван върху значката. **/
    displayLabel?: string | null
    /** Изображението, показвано за значката, ако използва такова. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Допълнителен CSS клас, приложен към значката. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP методи

Можете да конфигурирате HTTP метода за всеки тип уебкук събитие в администраторския панел:

- **Create Event**: POST или PUT (по подразбиране: PUT)
- **Update Event**: POST или PUT (по подразбиране: PUT)
- **Delete Event**: DELETE, POST или PUT (по подразбиране: DELETE)

Тъй като всички заявки съдържат ID, операциите Create и Update са идемпотентни по подразбиране (PUT). Повтарянето на една и съща заявка за Create или Update не трябва да създава дублирани обекти от ваша страна.

#### Заглавки на заявката

| Заглавка | Описание |
|----------|----------|
| `Content-Type` | `application/json` |
| `token` | Вашият API Secret |
| `X-FastComments-Timestamp` | Unix времева отметка (секунди), когато заявката е подписана |
| `X-FastComments-Signature` | HMAC-SHA256 подпис (`sha256=<hex>`) |

Вижте [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) за информация относно проверка на HMAC подписа.