`UserBadge` FastComments sisteminde bir kullanıcıya atanan bir rozeti temsil eden bir nesnedir.

Rozetler, kullanıcı etkinliğine (such as comment count, reply time, veteran status) göre otomatik olarak veya site yöneticileri tarafından elle atanabilir.

`The structure for the `UserBadge` object is as follows:` -> `UserBadge` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'UserBadge Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Bu kullanıcı rozet atamasının benzersiz tanımlayıcı */
    id: string
    /** Bu rozetin atandığı kullanıcının ID'si */
    userId: string
    /** Kiracının rozet kataloğundan rozet tanımının ID'si */
    badgeId: string
    /** Bu rozeti oluşturan/atan kiracının ID'si */
    fromTenantId: string
    /** Bu rozetin oluşturulduğu zaman (epoch'tan bu yana milisaniye) */
    createdAt?: number
    /** Bu rozetin kullanıcı tarafından alındığı zaman (epoch'tan bu yana milisaniye) */
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
    /** Eşiğe dayalı rozetler için eşik değeri */
    threshold?: number
    /** Rozetin adı/etiketi */
    name?: string
    /** Rozetin ayrıntılı açıklaması */
    description?: string
    /** Rozette gösterilen metin */
    displayLabel?: string
    /** Rozette gösterilen resmin URL'si */
    displaySrc?: string
    /** Rozet için arka plan rengi (hex kodu) */
    backgroundColor?: string
    /** Rozet için kenarlık rengi (hex kodu) */
    borderColor?: string
    /** Rozet için metin rengi (hex kodu) */
    textColor?: string
    /** Stil için ek CSS sınıfı */
    cssClass?: string
    /** Veteran rozetleri için zaman eşiği (milisaniye) */
    veteranUserThresholdMillis?: number
    /** Bu rozetin kullanıcının yorumlarında görüntülenip görüntülenmediği */
    displayedOnComments: boolean
    /** Rozetin gösterim sırası */
    order?: number
}
[inline-code-end]