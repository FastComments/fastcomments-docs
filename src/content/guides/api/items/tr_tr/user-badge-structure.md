`UserBadge`, FastComments sisteminde bir kullanıcıya atanan rozeti temsil eden bir nesnedir.

Rozetler, kullanıcı etkinliğine (örneğin yorum sayısı, yanıt süresi, veteran durumu) göre otomatik olarak veya site yöneticileri tarafından manuel olarak kullanıcılara atanabilir.

`UserBadge` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'UserBadge Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Bu kullanıcı rozet atamasının benzersiz tanımlayıcısı */
    id: string
    /** Bu rozeti alan kullanıcının ID'si */
    userId: string
    /** Tenant'ın rozet kataloğundan rozet tanımının ID'si */
    badgeId: string
    /** Bu rozeti oluşturan/atan tenant'ın ID'si */
    fromTenantId: string
    /** Bu rozetin oluşturulduğu zaman (epoch'tan itibaren milisaniye) */
    createdAt?: number
    /** Bu rozetin kullanıcı tarafından alındığı zaman (epoch'tan itibaren milisaniye) */
    receivedAt?: number
    /** 
     * Rozet türü: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Eşik tabanlı rozetler için eşik değeri */
    threshold?: number
    /** Rozetin adı/etiketi */
    name?: string
    /** Rozetin ayrıntılı açıklaması */
    description?: string
    /** Rozette gösterilen metin */
    displayLabel?: string
    /** Rozette gösterilen görselin URL'si */
    displaySrc?: string
    /** Rozet için arka plan rengi (hex kodu) */
    backgroundColor?: string
    /** Rozet için çerçeve rengi (hex kodu) */
    borderColor?: string
    /** Rozet için metin rengi (hex kodu) */
    textColor?: string
    /** Stil için ek CSS sınıfı */
    cssClass?: string
    /** Veteran rozetleri için zaman eşiği (milisaniye cinsinden) */
    veteranUserThresholdMillis?: number
    /** Bu rozetin kullanıcının yorumlarında gösterilip gösterilmediği */
    displayedOnComments: boolean
    /** Rozetin gösterim sırası */
    order?: number
    /** Belirlenmişse, bu rozet yalnızca eşleşen urlId'ye sahip sayfada gösterilir. Genel rozetler için null. */
    urlId?: string | null
}
[inline-code-end]
---