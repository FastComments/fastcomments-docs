`UserBadgeProgress` est un objet qui représente la progression d'un utilisateur vers l'obtention de divers badges dans le système FastComments.

Ce suivi aide à déterminer quand les utilisateurs devraient recevoir des badges automatiques en fonction de leur activité et participation dans votre communauté.

La structure de l'objet `UserBadgeProgress` est la suivante :

[inline-code-attrs-start title = 'Structure de UserBadgeProgress'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Unique identifier for this progress record */
    id: string
    /** ID of the tenant this progress record belongs to */
    tenantId: string
    /** ID of the user this progress record tracks */
    userId: string
    /** ID of the user's first comment in the system */
    firstCommentId?: string
    /** Date of the user's first comment (milliseconds since epoch) */
    firstCommentDate?: number
    /** Automatically calculated trust factor based on user activity */
    autoTrustFactor?: number
    /** Manually set trust factor by administrators */
    manualTrustFactor?: number
    /** Detailed progress object with various metrics, keys match BadgeType enum */
    progress: {
        /** 0: CommentCount - Count of comments the user has made */
        '0'?: number
        /** 1: CommentUpVotes - Count of upvotes the user has received */
        '1'?: number
        /** 2: CommentReplies - Count of replies the user has made */
        '2'?: number
        /** 3: CommentsPinned - Count of pinned comments the user has */
        '3'?: number
        /** 4: Veteran - User's account age */
        '4'?: number
        /** 5: NightOwl - Times user has posted during nighttime hours */
        '5'?: number
        /** 6: FastReplyTime - Average reply time in milliseconds */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - For moderator badges, comments deleted count */
        '7'?: number
        /** 8: ModeratorCommentsApproved - For moderator badges, comments approved count */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - For moderator badges, comments unapproved count */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - For moderator badges, comments reviewed count */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - For moderator badges, comments marked as spam count */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - For moderator badges, comments marked as not spam count */
        '12'?: number
        /** 13: RepliedToSpecificPage - For each page, count of replies */
        '13'?: Record<string, number>
    }
}
[inline-code-end]
