Bir `Comment` nesnesi, bir kullanıcı tarafından bırakılan bir yorumu temsil eder.

Üst ve alt yorumlar arasındaki ilişki `parentId` üzerinden tanımlanır.

Comment nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'Comment Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Spam motoru yorumun spam olduğunu belirlediyse true olarak ayarlanır. **/
    aiDeterminedSpam?: boolean
    /** Yorumun gösterilmesi onaylandı mı. Yorum kaydedilirken true olarak ayarlanır, aksi halde gizlenir. **/
    approved?: boolean
    /** Kullanıcının avatarı. **/
    avatarSrc?: string
    /** Alt yorumlar. Her senaryoda doldurulmaz. API üzerinden asTree true olarak ayarlandığında kullanılır. **/
    children: Comment[]
    /** Yorum yapan kişinin ham yorumu. **/
    comment: string
    /** READONLY: Yorum yapan kişinin yorumu HTML'e dönüştürülmüş hali. **/
    commentHTML?: string
    /** Yorum yapan kişinin e-postası. Anonim yorumlama kapalıysa gereklidir. **/
    commenterEmail?: string
    /** Yorum yapan kişinin bağlantısı (örneğin blogu). **/
    commenterLink?: string
    /** Yorum yapan kişinin adı. Her zaman gereklidir. Mevcut değilse "Anonymous" gibi bir değere ayarlanır. **/
    commenterName: string
    /** Yorumun bırakıldığı tarih, UTC epoch cinsinden. **/
    date: number
    /** Yorum için "görüntüleme etiketi" - örneğin "Admin", "Moderator" veya "VIP User" gibi. **/
    displayLabel?: string
    /** Yorumun gönderildiği alan adı. **/
    domain?: string
    /** READONLY: Yorumun bayrak eklenme sayısı. **/
    flagCount?: number
    /** Yorumda yazılan ve başarıyla ayrıştırılan #hashtags. Sorgulama amacıyla manuel olarak da hashtag ekleyebilirsiniz, ancak bunlar yorum metninde otomatik olarak görüntülenmez. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Yorum görseller içeriyor mu? **/
    hasImages?: boolean
    /** READONLY: Yorum bağlantılar içeriyor mu? **/
    hasLinks?: boolean
    /** READONLY: Benzersiz yorum kimliği. **/
    id: string
    /** Sadece oluşturma sırasında! Bu değer depolama için hash'lenir. **/
    ip?: string
    /** READONLY: Mevcut kullanıcı, bu yorumu yazan kullanıcıyı engelledi mi? **/
    isBlocked?: boolean
    /** READONLY: Yorum bir yönetici tarafından mı yapıldı? userId'ye göre otomatik olarak ayarlanır. **/
    isByAdmin?: boolean
    /** READONLY: Yorum bir moderatör tarafından mı yapıldı? userId'ye göre otomatik olarak ayarlanır. **/
    isByModerator?: boolean
    /** Yorum yumuşak silindi ise true olarak ayarlanır (başka bir yapılandırma nedeniyle yer tutucu bırakılması gerekti). **/
    isDeleted?: boolean
    /** Kullanıcının hesabı silindiğinde ve yorumun korunması gerektiğinde true olarak ayarlanır. **/
    isDeletedUser?: boolean
    /** READONLY: Şu anda oturum açmış kullanıcı (contextUserId) tarafından bayraklandı mı? **/
    isFlagged?: boolean
    /** Yorum sabitlendi mi? **/
    isPinned?: boolean
    /** Yorum kilitli mi? True olduğunda, kilidi açılana kadar kimse (moderatörler dahil) yanıt veremez, düzenleyemez veya silemez. **/
    isLocked?: boolean
    /** Yorum spam mi? **/
    isSpam?: boolean
    /** READONLY: Yorum, mevcut kullanıcı (contextUserId) için aşağı oylandı mı? **/
    isVotedDown?: boolean
    /** READONLY: Yorum, mevcut kullanıcı (contextUserId) için yukarı oylandı mı? **/
    isVotedUp?: boolean
    /** Yorumun bulunduğu yerel ayar. Sağlanmazsa, HTTP accept-language başlığından türetilir. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: Yorumda yazılan ve başarıyla ayrıştırılan @mentions. **/
    mentions?: CommentUserMention[]
    /** Yorumla ilişkili isteğe bağlı meta veriler. **/
    meta?: Record<string, string | number | boolean>
    /** Bu yorumla ilişkili isteğe bağlı moderasyon grubu kimlikleri listesi. **/
    moderationGroupIds?: string[]|null
    /** READONLY: Bu yorum üzerindeki mevcut kullanıcının (contextUserId) oylamasına karşılık gelen oy nesnesinin kimliği. **/
    myVoteId?: string
    /** Bu yorum için yorum yapanlara bildirim gönderilip gönderilmediği. İçe aktarımlarda bildirim gönderilmesini önlemek için bunu true olarak ayarlayın. **/
    notificationSentForParent?: boolean
    /** Bu yorum için kiracı (tenant) kullanıcılarına bildirim gönderilip gönderilmediği. İçe aktarımlarda bildirim gönderilmesini önlemek için bunu true olarak ayarlayın. **/
    notificationSentForParentTenant?: boolean
    /** Bu yorumun bulunduğu sayfanın başlığı. **/
    pageTitle?: string
    /** Bir yoruma yanıt veriyorsak, bu yanıt verdiğimiz yorumun kimliğidir. **/
    parentId?: string|null
    /** Yorum incelendi olarak işaretlendi mi. **/
    reviewed: boolean
    /** Yorumun ait olduğu kiracı (tenant) kimliği. **/
    tenantId: string
    /** Yorumu yazan kullanıcı. Ad/e-posta ile bir yorum kaydedilirken otomatik olarak oluşturulur. **/
    userId?: string|null
    /** Bu yorumun göründüğü konumun URL'si, örneğin bir blog yazısı. **/
    url: string
    /** Bize ilettiğiniz urlId'nin "temizlenmiş" bir sürümü. Kaydederken bu alanı belirtirsiniz, ancak yorumu geri getirdiğinizde bu "temizlenmiş" olur ve orijinal değeriniz "urlIdRaw" alanına taşınır. **/
    urlId: string
    /** READONLY: Bize ilettiğiniz orijinal urlId. **/
    urlIdRaw?: string
    /** Kullanıcı ve bu yorum doğrulanmış mı? **/
    verified: boolean
    /** Yukarı oy sayısı. **/
    votesUp?: number
    /** Aşağı oy sayısı. **/
    votesDown?: number
    /** Yorumun "karma" değeri (= yukarı oylar - aşağı oylar). **/
    votes?: number
}
[inline-code-end]

Bu alanlardan bazıları `READONLY` olarak işaretlenmiştir - bunlar API tarafından döndürülür ancak ayarlanamaz.

### Yorum Metni Yapısı

Yorumlar, FastComments'a özgü bir markdown lehçesiyle yazılır; bu, markdown'a artı görseller için geleneksel `bbcode` tarzı etiketlerin eklenmiş halidir, örneğin `[img]path[/img]`.

Metin iki alanda saklanır. Kullanıcının girdiği metin, `comment` alanında değiştirilmeden saklanır. Bu, işlenip `commentHTML` alanında saklanır.

İzin verilen HTML etiketleri `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, ve br`'dir.

HTML'i işlemesi önerilir, çünkü çok küçük bir HTML alt kümesidir ve bir işleyici (renderer) oluşturmak oldukça basittir. Örneğin, React Native ve Flutter için bununla yardımcı olabilecek birden fazla kütüphane vardır.

`comment` alanının normalize edilmemiş değerini işlemeyi seçebilirsiniz. [Örnek bir ayrıştırıcı burada.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Örnek ayrıştırıcı, HTML ile çalışacak ve HTML etiketlerini platformunuz için işlenecek beklenen öğelere dönüştürecek şekilde de uyarlanabilir.

### Etiketleme

Kullanıcılar bir yorumda etiketlendiğinde, bilgi `mentions` adlı bir listede saklanır. Bu listedeki her nesne
aşağıdaki yapıya sahiptir.

[inline-code-attrs-start title = 'Comment Mentions Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Kullanıcı kimliği. SSO kullanıcıları için, bu önüne kiracı (tenant) kimliğiniz eklenmiş şekilde olur. **/
    id: string
    /** @ sembolü dahil, nihai @mention etiket metni. **/
    tag: string
    /** @ sembolü dahil, orijinal @mention etiket metni. **/
    rawTag: string
    /** Etiketlenen kullanıcı türü. user = FastComments.com hesabı. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Kullanıcı bildirimlerden çıkış yapmış olsa bile, bu yine de true olarak ayarlanır. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Hashtag'ler kullanıldığında ve başarıyla ayrıştırıldığında, bilgi `hashTags` adlı bir listede saklanır. Bu listedeki her nesne
aşağıdaki yapıya sahiptir. `retain` ayarlanmışsa, hashtag'ler sorgulama amacıyla yorumun `hashTags` dizisine manuel olarak da eklenebilir.

[inline-code-attrs-start title = 'Comment HashTag Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** Hashtag kimliği. **/
    id: string
    /** # sembolü dahil, nihai #hashtag etiket metni. **/
    tag: string
    /** Hashtag özel bir URL ile ilişkilendirilmişse, bu tanımlı olacaktır. **/
    url?: string
    /** Yorum güncellendiğinde, hashtag yorum metninde bulunmasa bile korunması gerekiyorsa. Yorum metnini değiştirmeden yorumları etiketlemek için kullanışlıdır. **/
    retain?: boolean
}
[inline-code-end]