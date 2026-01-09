`UserBadgeProgress` is an object that represents a user's progress toward earning various badges in the FastComments system.

This tracking helps determine when users should receive automatic badges based on their activity and participation in your community.

The structure for the `UserBadgeProgress` object is as follows:

[inline-code-attrs-start title = 'UserBadgeProgress Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Bu ilerleme kaydı için benzersiz tanımlayıcı */
    id: string
    /** Bu ilerleme kaydının ait olduğu tenant ID'si */
    tenantId: string
    /** Bu ilerleme kaydının izlediği kullanıcının ID'si */
    userId: string
    /** Kullanıcının sistemdeki ilk yorumunun ID'si */
    firstCommentId?: string
    /** Kullanıcının ilk yorumunun tarihi (epoch'ten itibaren milisaniye cinsinden) */
    firstCommentDate?: number
    /** Kullanıcı etkinliğine göre otomatik hesaplanan güven faktörü */
    autoTrustFactor?: number
    /** Yöneticiler tarafından manuel olarak ayarlanan güven faktörü */
    manualTrustFactor?: number
    /** Çeşitli metrikler içeren ayrıntılı ilerleme nesnesi, anahtarlar BadgeType enum ile eşleşir */
    progress: {
        /** 0: CommentCount - Kullanıcının yaptığı yorum sayısı */
        '0'?: number
        /** 1: CommentUpVotes - Kullanıcının aldığı beğeni (upvote) sayısı */
        '1'?: number
        /** 2: CommentReplies - Kullanıcının yaptığı yanıt sayısı */
        '2'?: number
        /** 3: CommentsPinned - Kullanıcının sabitlenen yorum sayısı */
        '3'?: number
        /** 4: Veteran - Kullanıcının hesap yaşı */
        '4'?: number
        /** 5: NightOwl - Kullanıcının gece saatlerinde yorum yaptığı kez sayısı */
        '5'?: number
        /** 6: FastReplyTime - Ortalama yanıt süresi (milisaniye) */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - Moderatör rozetleri için, silinen yorum sayısı */
        '7'?: number
        /** 8: ModeratorCommentsApproved - Moderatör rozetleri için, onaylanan yorum sayısı */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - Moderatör rozetleri için, onaydan çıkarılan yorum sayısı */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - Moderatör rozetleri için, incelenen yorum sayısı */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - Moderatör rozetleri için, spam olarak işaretlenen yorum sayısı */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - Moderatör rozetleri için, spam olmayan olarak işaretlenen yorum sayısı */
        '12'?: number
        /** 13: RepliedToSpecificPage - Her sayfa için, yanıt sayıları */
        '13'?: Record<string, number>
    }
}
[inline-code-end]
---