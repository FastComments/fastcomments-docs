Webhook'lar aracılığıyla gönderilen tek yapı aşağıda TypeScript ile özetlenen WebhookComment nesnesidir.

#### WebhookComment Nesnesi Yapısı

##### "Create" Olay Yapısı
"create" olayının istek gövdesi bir WebhookComment nesnesidir.

##### "Update" Olay Yapısı
"update" olayının istek gövdesi bir WebhookComment nesnesidir.

##### "Delete" Olay Yapısı
"delete" olayının istek gövdesi bir WebhookComment nesnesidir.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'WebhookComment Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Yorumun id'si. **/
    id: string
    /** Yorumu tanımlayan id veya URL. Normalize edilmiş. **/
    urlId: string
    /** Yorumu bırakılan yere işaret eden URL. **/
    url?: string
    /** Yorumu bırakan kullanıcı id'si. SSO ise tenant id ile öneklenmiş. **/
    userId?: string
    /** Yorumu bırakan kullanıcının e-postası. **/
    commenterEmail?: string
    /** Yorum widget'ında görünen kullanıcı adı. SSO ile displayName olabilir. **/
    commenterName: string
    /** Ham yorum metni. **/
    comment: string
    /** Ayrıştırma sonrası yorum metni. **/
    commentHTML: string
    /** Yorumun dış id'si. **/
    externalId?: string
    /** Ebeveyn yorumun id'si. **/
    parentId?: string | null
    /** Yoruma bırakıldığı UTC tarih. **/
    date: UTC_ISO_DateString
    /** Oyların (yukarı - aşağı) birleşik karması. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Kullanıcı yorum yaparken giriş yapmışsa, yorumu doğrulanmışsa veya yorum bırakıldığında oturumunu doğrulamışsa true. **/
    verified: boolean
    /** Yorumun doğrulandığı tarih. **/
    verifiedDate?: number
    /** Bir moderatör yorumun incelendiğini işaretlediyse. **/
    reviewed: boolean
    /** Avatarın konumu veya base64 kodlaması. SSO ile base64 olarak gönderildiyse yalnızca o durumda base64 olur. **/
    avatarSrc?: string
    /** Yorum manuel veya otomatik olarak spam olarak işaretlendi mi? **/
    isSpam: boolean
    /** Yorum otomatik olarak spam olarak mı belirlendi? **/
    aiDeterminedSpam: boolean
    /** Yorumda resim var mı? **/
    hasImages: boolean
    /** "Most Relevant" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumber: number
    /** "Oldest First" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumberOF: number
    /** "Newest First" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumberNF: number
    /** Yorum otomatik veya manuel olarak onaylandı mı? **/
    approved: boolean
    /** Yorum yazılırken kullanıcının yerel kodu (format: en_us). **/
    locale: string
    /** Yorumda yazılan ve başarıyla ayrıştırılan @mention'lar. **/
    mentions?: CommentUserMention[]
    /** Yorumun geldiği domain. **/
    domain?: string
    /** Bu yorumla ilişkili isteğe bağlı moderasyon grup id'lerinin listesi. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Kullanıcılar bir yorumda etiketlendiğinde, bilgi `mentions` adlı bir listede saklanır. O listedeki her nesne aşağıdaki yapıya sahiptir.

[inline-code-attrs-start title = 'Webhook Mentions Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Kullanıcı id'si. SSO kullanıcıları için tenant id ile öneklenmiş olacaktır. **/
    id: string
    /** @ sembolü dahil, son @mention etiket metni. **/
    tag: string
    /** @ sembolü dahil, orijinal @mention etiket metni. **/
    rawTag: string
    /** Hangi tür kullanıcının etiketlendiği. user = FastComments.com hesabı. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Kullanıcı bildirimlerden çıkmış olsa bile, bu yine de true olarak ayarlanır. **/
    sent: boolean
}
[inline-code-end]

#### Kullanılan HTTP Yöntemleri

**Create ve Update her ikisi de HTTP PUT kullanır, POST değil!**

Tüm isteklerimiz bir ID içerdiğinden, aynı Create veya Update isteğini tekrar etmek tarafınızda yeni nesneler oluşturmaz.

Bu, bu çağrıların idempotent olduğu ve HTTP spesifikasyonuna göre PUT olayları olması gerektiği anlamına gelir.

---