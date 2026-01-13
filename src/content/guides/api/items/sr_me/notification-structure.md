Објекат `Notification` представља обавјештење за корисника.

`Notification` објекти се креирају аутоматски и не могу се креирати преко API-ja. Они такође истичу након једне године.
Обавјештења не могу бити обрисана. Међутим, могу се ажурирати тако да се `viewed` постави на `false`, и можете да правите упит по `viewed`.

Корисник такође може да се одкаже од обавјештења за одређени коментар постављањем `optedOut` у обавјештењу на `true`. Можете се поново пријавити постављањем на `false`.

Постоје различити типови обавјештења - погледајте `relatedObjectType` и `type`.

Начини на које се обавјештења креирају су прилично флексибилни и могу бити покренути у многим сценаријама (погледајте `NotificationType`).

У данашње вријеме, постојање `Notification` објекта не имплицира да је е-порука послата или да би требало да буде послата. Умјесто тога, обавјештења се користе за фид обавјештења и повезане интеграције.

Структура за објекат `Notification` је следећа:

[inline-code-attrs-start title = 'Структура обавјештења'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Ако вам је неко одговорио. **/
    RepliedToMe = 0,
    /** Ако је неко одговорио било гдје у нити (укључујући потомке више нивоа) на којој сте коментарисали. **/
    RepliedTransientChild = 1,
    /** Ако је ваш коментар добио позитиван глас. **/
    VotedMyComment = 2,
    /** Ако је нови коментар остављен на корјену странице на коју сте претплаћени. **/
    SubscriptionReplyRoot = 3,
    /** Ако је неко коментарисао на вашем профилу. **/
    CommentedOnProfile = 4,
    /** Ако имате директну поруку. **/
    DirectMessage = 5,
    /** TrialLimits is for tenant users only. **/
    TrialLimits = 6,
    /** Ако сте били @поменути. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** Са SSO-ом, user id је у формату `<tenant id>:<user id>`. **/
    userId?: string
    /** Када радите са SSO-ом, морате да бринете само о `userId`. **/
    anonUserId?: string
    /** urlId је готово увијек дефинисан. Он је опционалан само за обавјештења на нивоу tenant, што је ријетко. **/
    urlId?: string
    /** URL је кеширан за брзу навигацију до извора обавјештења. **/
    url?: string
    /** Наслов странице је кеширан за брзо читање извора обавјештења. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** На примјер, id коментара. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // низ датума
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName и fromUserAvatarSrc су кеширани овдје ради брзог приказивања обавјештења. Ажурирају се када се корисник ажурира. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Поставите ово на true да бисте престали примати обавјештења за овај објекат. **/
    optedOut?: boolean
}
[inline-code-end]