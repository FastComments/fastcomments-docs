`UserBadgeProgress` је објекат који представља напредак корисника ка стицању различитих значки у систему FastComments.

Ово праћење помаже у одређивању када корисници треба да добију аутоматске значке на основу своје активности и учешћа у вашој заједници.

Структура `UserBadgeProgress` објекта је следећа:

[inline-code-attrs-start title = 'Структура UserBadgeProgress'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Јединствени идентификатор за овај запис напретка */
    id: string
    /** ID тенанта коме припада овај запис напретка */
    tenantId: string
    /** ID корисника кога овај запис напретка прати */
    userId: string
    /** ID првог коментара корисника у систему */
    firstCommentId?: string
    /** Датум првог коментара корисника (милисекунде од епохе) */
    firstCommentDate?: number
    /** Аутоматски израчунати фактор поверења на основу активности корисника */
    autoTrustFactor?: number
    /** Фактор поверења подешен ручно од стране администратора */
    manualTrustFactor?: number
    /** Детаљан објекат напретка са различитим метрикама, кључеви одговарају BadgeType енуму */
    progress: {
        /** 0: CommentCount - Број коментара које је корисник написао */
        '0'?: number
        /** 1: CommentUpVotes - Број позитивних гласова (upvotes) које је корисник примио */
        '1'?: number
        /** 2: CommentReplies - Број одговора које је корисник дао */
        '2'?: number
        /** 3: CommentsPinned - Број причвршћених (pinned) коментара које корисник има */
        '3'?: number
        /** 4: Veteran - Старост корисничког налога */
        '4'?: number
        /** 5: NightOwl - Број пута када је корисник објавио током ноћних сати */
        '5'?: number
        /** 6: FastReplyTime - Просечно време одговора у милисекундама */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - За модераторске значке, број избрисаних коментара */
        '7'?: number
        /** 8: ModeratorCommentsApproved - За модераторске значке, број одобрених коментара */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - За модераторске значке, број неодобрених коментара */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - За модераторске значке, број прегледаних коментара */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - За модераторске значке, број коментара означених као спам */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - За модераторске значке, број коментара означених као да нису спам */
        '12'?: number
        /** 13: RepliedToSpecificPage - За сваку страницу, број одговора */
        '13'?: Record<string, number>
    }
}
[inline-code-end]
---