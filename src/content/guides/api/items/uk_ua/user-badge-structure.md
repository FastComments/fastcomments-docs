`UserBadge` — це об’єкт, який представляє бейдж, що призначається користувачу в системі FastComments.

Бейджі можуть призначатися користувачам автоматично на основі їхньої активності (наприклад, кількість коментарів, час відповіді, статус ветерана) або вручну адміністраторами сайту.

Структура об'єкта `UserBadge` наступна:

[inline-code-attrs-start title = 'Структура UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Унікальний ідентифікатор цього призначення бейджа користувачу */
    id: string
    /** ID користувача, якому призначено цей бейдж */
    userId: string
    /** ID визначення бейджа з каталогу бейджів орендаря */
    badgeId: string
    /** ID орендаря, який створив/призначив цей бейдж */
    fromTenantId: string
    /** Коли цей бейдж було створено (мілісекунд від початку епохи) */
    createdAt?: number
    /** Коли користувач отримав цей бейдж (мілісекунд від початку епохи) */
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
    /** Для бейджів на основі порога — значення порога */
    threshold?: number
    /** Назва/мітка бейджа */
    name?: string
    /** Детальний опис бейджа */
    description?: string
    /** Текст, що відображається на бейджі */
    displayLabel?: string
    /** URL зображення, що показується на бейджі */
    displaySrc?: string
    /** Колір фону для бейджа (шістнадцятковий код) */
    backgroundColor?: string
    /** Колір рамки для бейджа (шістнадцятковий код) */
    borderColor?: string
    /** Колір тексту для бейджа (шістнадцятковий код) */
    textColor?: string
    /** Додатковий CSS-клас для стилізації */
    cssClass?: string
    /** Для бейджів Veteran — пороговий час у мілісекундах */
    veteranUserThresholdMillis?: number
    /** Чи відображається цей бейдж на коментарях користувача */
    displayedOnComments: boolean
    /** Порядок відображення бейджа */
    order?: number
}
[inline-code-end]
---