---
`UserBadgeProgress` — це об'єкт, який представляє прогрес користувача щодо здобуття різних значків у системі FastComments.

Це відстеження допомагає визначити, коли користувачі повинні отримувати автоматичні значки на основі їхньої активності та участі у вашій спільноті.

Структура об'єкта `UserBadgeProgress` виглядає так:

[inline-code-attrs-start title = 'Структура UserBadgeProgress'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Унікальний ідентифікатор цього запису прогресу */
    id: string
    /** ID орендаря (tenant), якому належить цей запис прогресу */
    tenantId: string
    /** ID користувача, за яким відстежується цей запис прогресу */
    userId: string
    /** ID першого коментаря користувача в системі */
    firstCommentId?: string
    /** Дата першого коментаря користувача (мілісекунди від початку епохи) */
    firstCommentDate?: number
    /** Автоматично обчислений коефіцієнт довіри на основі активності користувача */
    autoTrustFactor?: number
    /** Коефіцієнт довіри, встановлений вручну адміністраторами */
    manualTrustFactor?: number
    /** Детальний об'єкт прогресу з різними метриками, ключі відповідають enum BadgeType */
    progress: {
        /** 0: CommentCount - Кількість коментарів, які залишив користувач */
        '0'?: number
        /** 1: CommentUpVotes - Кількість upvotes, які отримав користувач */
        '1'?: number
        /** 2: CommentReplies - Кількість відповідей, які надав користувач */
        '2'?: number
        /** 3: CommentsPinned - Кількість закріплених коментарів, які має користувач */
        '3'?: number
        /** 4: Veteran - Вік облікового запису користувача */
        '4'?: number
        /** 5: NightOwl - Кількість разів, коли користувач публікував у нічний час */
        '5'?: number
        /** 6: FastReplyTime - Середній час відповіді в мілісекундах */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - Для значків модератора, кількість видалених коментарів */
        '7'?: number
        /** 8: ModeratorCommentsApproved - Для значків модератора, кількість схвалених коментарів */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - Для значків модератора, кількість несхвалених коментарів */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - Для значків модератора, кількість переглянутих коментарів */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - Для значків модератора, кількість коментарів, позначених як спам */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - Для значків модератора, кількість коментарів, позначених як не спам */
        '12'?: number
        /** 13: RepliedToSpecificPage - Для кожної сторінки, кількість відповідей */
        '13'?: Record<string, number>
    }
}
[inline-code-end]
---