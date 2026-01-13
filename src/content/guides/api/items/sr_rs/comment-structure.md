A `Comment` object представља коментар који је оставио корисник.

Однос између родитељских и дечијих коментара је дефинисан преко `parentId`.

Структура Comment објекта је следећа:

[inline-code-attrs-start title = 'Struktura komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Поставити на true ако је spam механизам утврдио да је коментар спам. **/
    aiDeterminedSpam?: boolean
    /** Да ли је коментар одобрен за приказ. Поставити на true приликом чувања коментара, иначе ће бити скривен. **/
    approved?: boolean
    /** Корисников аватар. **/
    avatarSrc?: string
    /** Дечији коментари. Нису попуњени у свим сценаријима. Користе се када је asTree постављен на true преко API-ја. **/
    children: Comment[]
    /** Сирови коментар који је унео корисник. **/
    comment: string
    /** READONLY: Коментар корисника парсиран у HTML. **/
    commentHTML?: string
    /** Е-пошта коментатора. Обавезно ако је анонимно коментарисање искључено. **/
    commenterEmail?: string
    /** Линк коментатора (на пример, њихов блог). **/
    commenterLink?: string
    /** Име коментатора. Увек је обавезно. Ако није доступно, поставити нешто попут "Anonymous". **/
    commenterName: string
    /** Датум када је коментар остављен, у UTC epoch формату. **/
    date: number
    /** "Приказни ознака" за коментар - на пример "Admin", "Moderator", или нешто као "VIP User". **/
    displayLabel?: string
    /** Домен на ком је коментар објављен. **/
    domain?: string
    /** READONLY: Број пута колико је коментар пријављен. **/
    flagCount?: number
    /** Број #hahtag-ова написаних у коментару који су успешно парсирани. Такође можете ручно додавати хештегове за упите, али они се неће аутоматски приказати у тексту коментара. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Да ли коментар садржи слике? **/
    hasImages?: boolean
    /** READONLY: Да ли коментар садржи линкове? **/
    hasLinks?: boolean
    /** READONLY: Јединствени id коментара. **/
    id: string
    /** Само при креирању! Ово се хашује за складиштење. **/
    ip?: string
    /** READONLY: Да ли је тренутни корисник блокирао корисника који је написао овај коментар? **/
    isBlocked?: boolean
    /** READONLY: Да ли је коментар од администратора? Аутоматски се подешава на основу userId. **/
    isByAdmin?: boolean
    /** READONLY: Да ли је коментар од модератора? Аутоматски се подешава на основу userId. **/
    isByModerator?: boolean
    /** Поставити на true ако је коментар soft obrisan (морао је бити остављен плејсхолдер због неког другог подешавања). **/
    isDeleted?: boolean
    /** Поставити на true ако је кориснички налог обрисан, а коментар је морао бити сачуван. **/
    isDeletedUser?: boolean
    /** READONLY: Да ли је коментар означен од стране тренутно пријављеног корисника (contextUserId)? **/
    isFlagged?: boolean
    /** Да ли је коментар причвршћен (pinned)? **/
    isPinned?: boolean
    /** Да ли је коментар закључан за нове одговоре (модератори и даље могу одговорити)? **/
    isLocked?: boolean
    /** Да ли је коментар спам? **/
    isSpam?: boolean
    /** READONLY: Да ли је коментар негативно гласан за тренутног корисника (contextUserId)? **/
    isVotedDown?: boolean
    /** READONLY: Да ли је коментар позитивно гласан за тренутног корисника (contextUserId)? **/
    isVotedUp?: boolean
    /** Локал у којем је коментар. Ако није обезбеђен, изведе се из HTTP header-a за језик (accept). **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: @mentions написани у коментару који су успешно парсирани. **/
    mentions?: CommentUserMention[]
    /** Опционални метаподатак повезан са коментаром. **/
    meta?: Record<string, string | number | boolean>
    /** Опциона листа id-јева модерацијских група повезаних са овим коментаром. **/
    moderationGroupIds?: string[]|null
    /** READONLY: Ид гласа који одговара гласу од тренутног корисника (contextUserId) за овај коментар. **/
    myVoteId?: string
    /** Да ли су послате нотификације за овај коментар коментаторима. Да бисте спречили слање нотификација приликом увоза, поставите ово на true. **/
    notificationSentForParent?: boolean
    /** Да ли су послате нотификације за овај коментар tenant корисницима. Да бисте спречили слање нотификација приликом увоза, поставите ово на true. **/
    notificationSentForParentTenant?: boolean
    /** Наслов странице на којој се овај коментар налазио. **/
    pageTitle?: string
    /** Ако одговарамо на коментар, ово је ID на који одговарамо. **/
    parentId?: string|null
    /** Да ли је коментар означен као прегледан (reviewed). **/
    reviewed: boolean
    /** Tenant id где коментар припада. **/
    tenantId: string
    /** Корисник који је написао коментар. Аутоматски се креира када се сачува коментар са именом/имејлом. **/
    userId?: string|null
    /** URL локације на којој је овај коментар видљив, нпр. блог пост. **/
    url: string
    /** "Очишћена" верзија urlId коју сте нам проследили. При чувању овог поља уносите, али кад вратите коментар, ово ће бити "очишћено" а ваша оригинална вредност премештена у "urlIdRaw". **/
    urlId: string
    /** READONLY: Оригинални urlId који сте нам проследили. **/
    urlIdRaw?: string
    /** Да ли су корисник и овај коментар верификовани? **/
    verified: boolean
    /** Број позитивних гласова. **/
    votesUp?: number
    /** Број негативних гласова. **/
    votesDown?: number
    /** "Карма" коментара (= votes up - votes down). **/
    votes?: number
}
[inline-code-end]

Нека од ових поља су означена као `READONLY` - она се враћају од стране API-ја али не могу бити подешавана.

### Struktura teksta komentara

Коментари су написани у FastComments варијанти markdown-а, која је у основи markdown плус традиционалне bbcode стил тагове за слике, као што је `[img]path[/img]`.

Текст се чува у два поља. Текст који је корисник унео се чува непромењен у пољу `comment`. Ово се рендерује и чува у пољу `commentHTML`.

Дозвољени HTML тагови су `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Препоручује се рендеровање HTML-а, пошто је у питању врло мали подскуп HTML-а, па је прављење renderer-а прилично једноставно. Постоји више библиотека за React Native и Flutter, на пример, које могу помоћи у томе.

Можете изабрати да рендерујете нормализовану или ненормализовану вредност поља `comment`. [Пример парсера је овде.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Пример парсера се такође може прилагодити да ради са HTML-ом и трансформише HTML тагове у очекиване елементе за рендеровање на вашој платформи. 

### Obeležavanje

Када су корисници означени у коментару, информација се чува у листи која се зове `mentions`. Сваки објекат у тој листи
има следећу структуру.

[inline-code-attrs-start title = 'Objekat pomena u komentaru'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID корисника. За SSO кориснике, ово ће имати ваш tenant id као префикс. **/
    id: string
    /** Коначни @mention tag текст, укључујући @ симбол. **/
    tag: string
    /** Оригинални @mention tag текст, укључујући @ симбол. **/
    rawTag: string
    /** Која врста корисника је означена. user = FastComments.com налог. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ако се корисник одјави од нотификација, ово ће и даље бити постављено на true. **/
    sent: boolean
}
[inline-code-end]

### Haštagovi

Када се хештегови користе и успешно парсирају, информација се чува у листи која се зове `hashTags`. Сваки објекат у тој листи
има следећу структуру. Хештегови се такође могу ручно додати у низ `hashTags` коментара за упите, ако је `retain` постављен.

[inline-code-attrs-start title = 'Objekat haštaga u komentaru'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** ID хештега. **/
    id: string
    /** Коначни #hashtag tag текст, укључујући # симбол. **/
    tag: string
    /** Ако је хештег повезан са прилагођеним URL-ом, ово ће бити дефинисано. **/
    url?: string
    /** Ако треба да задржимо хештег, чак и ако он не постоји у тексту коментара када се коментар ажурира. Корисно за означавање коментара без мењања текста коментара. **/
    retain?: boolean
}
[inline-code-end]

---