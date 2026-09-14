`FeedPost` объект представляет запись в ленте FastComments. Лента — это поток записей со своими собственными ветками комментариев, отображаемый виджетом Feed. Каждая запись имеет автора, необязательный богатый контент, медиа и ссылки, и может быть помечена тегами, чтобы ленту можно было фильтровать.

Структура объекта `FeedPost` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** Идентификатор пользователя FastComments или SSO, который является автором записи. **/
    fromUserId?: string
    /** Заполняется из данных пользователя, если не задано. **/
    fromUserDisplayName?: string | null
    /** READONLY. Заполняется из данных пользователя. **/
    fromUserAvatar?: string | null
    /** Используется для фильтрации ленты. **/
    tags?: string[]
    /** Вес сортировки внутри ленты. Более высокие значения сортируются первыми. **/
    weight?: number
    /** Свободные пары ключ/значение для вашего собственного использования. **/
    meta?: Record<string, string>
    /** Очищенный HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Тип реакции для подсчёта. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Куда ведёт медиа‑элемент при клике. **/
    linkUrl?: string
    /** Одна запись для каждой версии. Виджет выбирает наиболее подходящую. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Текст ссылки, например «Зарегистрироваться сейчас». **/
    text?: string
    /** Заголовок, отображаемый вместе со ссылкой. **/
    title?: string
    /** Описание, отображаемое вместе со ссылкой. **/
    description?: string
    url?: string
}
[inline-code-end]

Примечания:

- Некоторые из этих полей помечены `READONLY` — они возвращаются API, но не могут быть установлены.
- Комментарии к записи являются обычными комментариями, чей `urlId` имеет вид `post:` + `_id` записи. Используйте это значение с API комментариев, чтобы читать или создавать комментарии к записи.