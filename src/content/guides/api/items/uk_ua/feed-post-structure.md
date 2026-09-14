A `FeedPost` object represents a post in a FastComments feed. A feed is a stream of posts with their own comment
threads, rendered by the Feed widget. Every post has an author, optional rich content, media, and links, and can be
tagged so that a feed can be filtered.

Об’єкт `FeedPost` представляє пост у стрічці FastComments. Стрічка — це потік постів зі своїми власними гілками коментарів, які відображаються за допомогою віджету Feed. Кожен пост має автора, необов’язковий багатий вміст, медіа та посилання, і може бути позначений тегами, щоб стрічку можна було фільтрувати.

The structure for the `FeedPost` object is as follows:

Структура об’єкта `FeedPost` виглядає наступним чином:

[inline-code-attrs-start title = 'Структура FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** Ідентифікатор користувача FastComments або SSO, який створив пост. **/
    fromUserId?: string
    /** Заповнюється з даних користувача, якщо не встановлено. **/
    fromUserDisplayName?: string | null
    /** READONLY. Заповнюється з даних користувача. **/
    fromUserAvatar?: string | null
    /** Використовується для фільтрації стрічки. **/
    tags?: string[]
    /** Вага сортування в межах стрічки. Вищі значення сортуються першими. **/
    weight?: number
    /** Пари ключ/значення довільного формату для вашого використання. **/
    meta?: Record<string, string>
    /** Очищений HTML. **/
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
    /** Куди посилається медіа‑елемент при кліку. **/
    linkUrl?: string
    /** Один запис на кожну варіацію. Віджет вибирає найкращий варіант. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Текст посилання, наприклад "Sign up now". **/
    text?: string
    /** Заголовок, що відображається разом з посиланням. **/
    title?: string
    /** Опис, що відображається разом з посиланням. **/
    description?: string
    url?: string
}
[inline-code-end]

Notes:

Примітки:

- Деякі з цих полів позначені `READONLY` — вони повертаються API, але не можуть бути встановлені.
- Коментарі до поста є звичайними коментарями, у яких `urlId` має вигляд `post:` + `_id` поста. Використовуйте це значення з Comment API, щоб читати або створювати коментарі до поста.