`UserBadgeProgress`, bir kullanıcının FastComments sistemindeki çeşitli rozetleri kazanmaya yönelik ilerlemesini temsil eden bir nesnedir.

Bu takip, kullanıcıların topluluğunuzdaki etkinliklerine ve katılımlarına göre ne zaman otomatik rozet alması gerektiğini belirlemeye yardımcı olur.

`UserBadgeProgress` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'UserBadgeProgress Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Bu ilerleme kaydının benzersiz tanımlayıcısı */
    id: string
    /** Bu ilerleme kaydının ait olduğu kiracı (tenant) kimliği */
    tenantId: string
    /** Bu ilerleme kaydının takip ettiği kullanıcının kimliği */
    userId: string
    /** Kullanıcının sistemdeki ilk yorumunun kimliği */
    firstCommentId?: string
    /** Kullanıcının ilk yorumunun tarihi (epoch'tan beri geçen milisaniye) */
    firstCommentDate?: number
    /** Kullanıcı etkinliğine göre otomatik hesaplanan güven faktörü */
    autoTrustFactor?: number
    /** Yöneticiler tarafından manuel olarak ayarlanan güven faktörü */
    manualTrustFactor?: number
    /** Çeşitli metriklerle ilgili ayrıntılı ilerleme nesnesi, anahtarlar BadgeType enum ile eşleşir */
    progress: {
        /** 0: CommentCount - Kullanıcının yaptığı yorum sayısı */
        '0'?: number
        /** 1: CommentUpVotes - Kullanıcının aldığı beğeni (upvote) sayısı */
        '1'?: number
        /** 2: CommentReplies - Kullanıcının yaptığı yanıt sayısı */
        '2'?: number
        /** 3: CommentsPinned - Kullanıcının sabitlenmiş yorum sayısı */
        '3'?: number
        /** 4: Veteran - Kullanıcının hesap yaşı */
        '4'?: number
        /** 5: NightOwl - Kullanıcının gece saatlerinde paylaşım yapma sayısı */
        '5'?: number
        /** 6: FastReplyTime - Milisaniye cinsinden ortalama yanıt süresi */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - Moderatör rozetleri için, silinen yorum sayısı */
        '7'?: number
        /** 8: ModeratorCommentsApproved - Moderatör rozetleri için, onaylanan yorum sayısı */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - Moderatör rozetleri için, onaydan kaldırılan yorum sayısı */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - Moderatör rozetleri için, incelenen yorum sayısı */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - Moderatör rozetleri için, spam olarak işaretlenen yorum sayısı */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - Moderatör rozetleri için, spam değil olarak işaretlenen yorum sayısı */
        '12'?: number
        /** 13: RepliedToSpecificPage - Her sayfa için yanıt sayısı */
        '13'?: Record<string, number>
    }
}
[inline-code-end]