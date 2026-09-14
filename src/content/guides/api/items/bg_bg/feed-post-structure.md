A `FeedPost` обект представлява публикация във FastComments емисия. Емисията е поток от публикации със свои собствени нишки за коментари, визуализирани от Feed уиджета. Всяка публикация има автор, по избор богато съдържание, медия и връзки, и може да бъде етикетирана, за да може емисията да се филтрира.

Структурата за обекта `FeedPost` е следната:

[inline-code-attrs-start title = 'FeedPost Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** Идентификаторът на FastComments или SSO потребителя, който е създал публикацията. **/
    fromUserId?: string
    /** Попълнено от потребителя, ако не е зададено. **/
    fromUserDisplayName?: string | null
    /** READONLY. Попълнено от потребителя. **/
    fromUserAvatar?: string | null
    /** Използва се за филтриране на емисия. **/
    tags?: string[]
    /** Тегло за сортиране в емисия. По-големи стойности се сортират първи. **/
    weight?: number
    /** Свободни двойки ключ/стойност за ваша употреба. **/
    meta?: Record<string, string>
    /** Санитизиран HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Reaction type to count. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Къде води медийният елемент при кликване. **/
    linkUrl?: string
    /** По един запис за всяка версия. Уиджетът избира най-подходящата. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Текстът на връзката, например "Sign up now". **/
    text?: string
    /** Заглавие, показано с връзката. **/
    title?: string
    /** Описание, показано с връзката. **/
    description?: string
    url?: string
}
[inline-code-end]

Бележки:

- Някои от тези полета са маркирани като `READONLY` – те се връщат от API, но не могат да бъдат зададени.
- Коментарите към публикация са обикновени коментари, чийто `urlId` е `post:` последвано от `_id` на публикацията. Използвайте тази стойност с Comment API, за да прочетете или създадете коментари към публикацията.