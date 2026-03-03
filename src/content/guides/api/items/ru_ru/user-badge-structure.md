`UserBadge` — объект, который представляет бейдж, присвоенный пользователю в системе FastComments.

Бейджи могут присваиваться пользователям автоматически на основе их активности (таких как количество комментариев, время ответа, статус ветерана) или вручную администраторами сайта.

Структура объекта `UserBadge` следующая:

[inline-code-attrs-start title = 'Структура UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Уникальный идентификатор присвоения бейджа пользователю */
    id: string
    /** ID пользователя, которому присвоен этот бейдж */
    userId: string
    /** ID определения бейджа из каталога бейджей тенанта */
    badgeId: string
    /** ID тенанта, который создал/присвоил этот бейдж */
    fromTenantId: string
    /** Когда этот бейдж был создан (в миллисекундах с начала эпохи) */
    createdAt?: number
    /** Когда этот бейдж был получен пользователем (в миллисекундах с начала эпохи) */
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
    /** Для бейджей на основе порога — значение порога */
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
    /** Цвет текста на бейдже (hex-код) */
    textColor?: string
    /** Дополнительный CSS-класс для стилизации */
    cssClass?: string
    /** Для бейджей "ветеран" — порог времени в миллисекундах */
    veteranUserThresholdMillis?: number
    /** Отображается ли этот бейдж в комментариях пользователя */
    displayedOnComments: boolean
    /** Порядок отображения бейджа */
    order?: number
    /** Если задано, этот бейдж отображается только на странице с соответствующим urlId. Null для глобальных бейджей. */
    urlId?: string | null
}
[inline-code-end]
---