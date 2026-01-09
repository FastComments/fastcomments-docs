Објекат `Notification` представља обавештење за корисника.

`Notification` објекти се креирају аутоматски и не могу се креирати преко API-ја. Такође истичу након једне године.
Обавештења се не могу обрисати. Међутим, могу се ажурирати тако да се `viewed` постави на `false`, и можете их упитати по `viewed`.

Корисник такође може да се искључи из обавештења за одређени коментар постављањем `optedOut` у обавештењу на `true`. Поново се можете укључити постављањем на `false`.

Постоје различите врсте обавештења — проверите `relatedObjectType` и `type`.

Начини на које се обавештења креирају су прилично флексибилни и могу бити покренути у разним сценаријима (видети `NotificationType`).

У данашњем систему, постојање `Notification` не подразумева да је имејл послат или да треба да буде послат. Уместо тога, обавештења се користе за notification feed и повезане интеграције.

Структура објекта `Notification` је следећа:

[inline-code-attrs-start title = 'Структура обавештења'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Ако вам је неко одговорио. **/
    RepliedToMe = 0,
    /** Ако је неко одговорио било где у нити (чак и у под-нитима) у нити на коју сте коментарисали. **/
    RepliedTransientChild = 1,
    /** Ако је ваш коментар добио позитиван глас. **/
    VotedMyComment = 2,
    /** Ако је на корен странице на коју сте претплаћени остављен нови коментар. **/
    SubscriptionReplyRoot = 3,
    /** Ако је неко коментарисао на вашем профилу. **/
    CommentedOnProfile = 4,
    /** Ако имате директну поруку (DM). **/
    DirectMessage = 5,
    /** TrialLimits је само за кориснике тенанта. **/
    TrialLimits = 6,
    /** Ако сте били @поменути. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** Код SSO, user id има формат `<tenant id>:<user id>`. **/
    userId?: string
    /** Када радите са SSO, треба да бринете само о `userId`. **/
    anonUserId?: string
    /** `urlId` је готово увек дефинисан. Опционо је само за обавештења на нивоу тенанта, што је ретко. **/
    urlId?: string
    /** URL се кешира ради брзог навигације до извора обавештења. **/
    url?: string
    /** Наслов странице се кешира ради брзог прегледа извора обавештења. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** На пример, id коментара. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // датумски низ
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName и fromUserAvatarSrc су овде кеширани ради брзог приказивања обавештења. Ажурирају се када се корисник ажурира. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Поставите ово на true да бисте престали да примате обавештења за овај објекат. **/
    optedOut?: boolean
}
[inline-code-end]

---