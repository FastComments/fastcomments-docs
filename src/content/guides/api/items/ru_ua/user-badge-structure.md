`UserBadge` — это объект, представляющий бейдж, присвоенный пользователю в системе FastComments.

Бейджи могут присваиваться пользователям автоматически на основе их активности (например, количества комментариев, времени ответа, статуса Veteran) или вручную администраторами сайта.

Структура объекта `UserBadge` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Уникальный идентификатор этого назначения бейджа пользователю */
    id: string
    /** ID пользователя, которому присвоен этот бейдж */
    userId: string
    /** ID определения бейджа из каталога бейджей арендатора */
    badgeId: string
    /** ID арендатора, который создал/присвоил этот бейдж */
    fromTenantId: string
    /** Когда этот бейдж был создан (миллисекунды с начала эпохи) */
    createdAt?: number
    /** Когда этот бейдж был получен пользователем (миллисекунды с начала эпохи) */
    receivedAt?: number
    /** 
     * Тип бейджа: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Для бейджей, основанных на пороге, значение порога */
    threshold?: number
    /** Название/метка бейджа */
    name?: string
    /** Подробное описание бейджа */
    description?: string
    /** Текст, отображаемый на бейдже */
    displayLabel?: string
    /** URL изображения, отображаемого на бейдже */
    displaySrc?: string
    /** Цвет фона бейджа (hex-код) */
    backgroundColor?: string
    /** Цвет границы бейджа (hex-код) */
    borderColor?: string
    /** Цвет текста бейджа (hex-код) */
    textColor?: string
    /** Дополнительный CSS-класс для стилизации */
    cssClass?: string
    /** Для бейджей типа Veteran, порог времени в миллисекундах */
    veteranUserThresholdMillis?: number
    /** Отображается ли этот бейдж на комментариях пользователя */
    displayedOnComments: boolean
    /** Порядок отображения бейджа */
    order?: number
}
[inline-code-end]