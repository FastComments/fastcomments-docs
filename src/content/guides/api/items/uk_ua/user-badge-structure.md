`UserBadge` — це об'єкт, який представляє бейдж, призначений користувачеві в системі FastComments.

Бейджі можуть призначатися користувачам автоматично на основі їхньої активності (наприклад, кількість коментарів, час відповіді, статус ветерана) або вручну адміністраторами сайту.

Структура об'єкта `UserBadge` така:

[inline-code-attrs-start title = 'Структура UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Унікальний ідентифікатор цього призначення бейджа користувачу */
    id: string
    /** ID користувача, якому призначено цей бейдж */
    userId: string
    /** ID визначення бейджа з каталогу бейджів тенанта */
    badgeId: string
    /** ID тенанта, який створив/призначив цей бейдж */
    fromTenantId: string
    /** Коли цей бейдж було створено (мілісекунди від початку епохи) */
    createdAt?: number
    /** Коли цей бейдж було отримано користувачем (мілісекунди від початку епохи) */
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
    /** Для бейджів, що залежать від порогу, значення порога */
    threshold?: number
    /** Назва/мітка бейджа */
    name?: string
    /** Детальний опис бейджа */
    description?: string
    /** Текст, відображений на бейджі */
    displayLabel?: string
    /** URL до зображення, що відображається на бейджі */
    displaySrc?: string
    /** Колір фону бейджа (шістнадцятковий код) */
    backgroundColor?: string
    /** Колір рамки бейджа (шістнадцятковий код) */
    borderColor?: string
    /** Колір тексту на бейджі (шістнадцятковий код) */
    textColor?: string
    /** Додатковий CSS-клас для стилізації */
    cssClass?: string
    /** Для бейджів ветерана — часовий поріг у мілісекундах */
    veteranUserThresholdMillis?: number
    /** Чи відображається цей бейдж у коментарях користувача */
    displayedOnComments: boolean
    /** Порядок відображення бейджа */
    order?: number
    /** Якщо встановлено, цей бейдж відображається лише на сторінці з відповідним urlId. Null для глобальних бейджів. */
    urlId?: string | null
}
[inline-code-end]