---
`UserBadge` — объект, представляющий значок, присвоенный пользователю в системе FastComments.

Значки могут присваиваться пользователям автоматически в зависимости от их активности (например, количества комментариев, времени ответа, статуса ветерана) или вручную администраторами сайта.

Структура объекта `UserBadge` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Уникальный идентификатор присвоения значка пользователю */
    id: string
    /** ID пользователя, которому присвоен этот значок */
    userId: string
    /** ID определения значка из каталога значков тенанта */
    badgeId: string
    /** ID тенанта, который создал/назначил этот значок */
    fromTenantId: string
    /** Когда этот значок был создан (миллисекунды с эпохи) */
    createdAt?: number
    /** Когда пользователь получил этот значок (миллисекунды с эпохи) */
    receivedAt?: number
    /** 
     * Тип значка: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Для значков на основе порога — пороговое значение */
    threshold?: number
    /** Название/метка значка */
    name?: string
    /** Подробное описание значка */
    description?: string
    /** Текст, отображаемый на значке */
    displayLabel?: string
    /** URL изображения, отображаемого на значке */
    displaySrc?: string
    /** Фоновый цвет значка (hex-код) */
    backgroundColor?: string
    /** Цвет рамки значка (hex-код) */
    borderColor?: string
    /** Цвет текста значка (hex-код) */
    textColor?: string
    /** Дополнительный CSS-класс для стилизации */
    cssClass?: string
    /** Для «ветеранских» значков — порог времени в миллисекундах */
    veteranUserThresholdMillis?: number
    /** Отображается ли этот значок в комментариях пользователя */
    displayedOnComments: boolean
    /** Порядок отображения значка */
    order?: number
    /** Если установлено, этот значок отображается только на странице с совпадающим urlId. Null для глобальных значков. */
    urlId?: string | null
}
[inline-code-end]
---