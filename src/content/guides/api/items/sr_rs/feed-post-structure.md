A `FeedPost` објекат представља пост у FastComments фиду. Фид је ток постова са сопственим темама коментара, приказаним помоћу Feed виџета. Сваки пост има аутора, опционо богати садржај, медије и линкове, и може бити означен тако да се фид може филтрирати.

Структура за `FeedPost` објекат је следећа:

[inline-code-attrs-start title = 'Структура FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** САМО ЗА ЧИТАЊЕ **/
    _id: string
    /** САМО ЗА ЧИТАЊЕ **/
    tenantId: string
    title?: string
    /** ИД FastComments или SSO корисника који је аутор поста. **/
    fromUserId?: string
    /** Попуњено из корисника када није постављено. **/
    fromUserDisplayName?: string | null
    /** САМО ЗА ЧИТАЊЕ. Попуњено из корисника. **/
    fromUserAvatar?: string | null
    /** Користи се за филтрирање фида. **/
    tags?: string[]
    /** Тежина сортирања унутар фида. Веће вредности се сортирају прве. **/
    weight?: number
    /** Парови кључ/вредност слободног формата за вашу употребу. **/
    meta?: Record<string, string>
    /** Санитизовани HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** САМО ЗА ЧИТАЊЕ **/
    createdAt: string
    /** САМО ЗА ЧИТАЊЕ. Тип реакције за број. **/
    reacts?: Record<string, number>
    /** САМО ЗА ЧИТАЊЕ **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Где медијски елемент води када се кликне. **/
    linkUrl?: string
    /** Један унос по рендацију. Виџет бира најбољу верзију. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Текст линка, нпр. „Sign up now“. **/
    text?: string
    /** Наслов приказан уз линк. **/
    title?: string
    /** Опис приказан уз линк. **/
    description?: string
    url?: string
}
[inline-code-end]

Напомене:

- Нека од ових поља су означена као `READONLY` – они се враћају из API‑ја, али се не могу поставити.
- Коментари на посту су обични коментари чији је `urlId` `post:` праћен `_id` поста. Користите ту вредност са Comment API‑јем да прочитате или креирате коментаре на посту.