Webhooklar aracılığıyla gönderilen tek yapı, aşağıda TypeScript ile özetlenen WebhookComment nesnesidir.

#### WebhookComment Nesnesinin Yapısı

##### "Create" Olayının Yapısı
"create" olayının istek gövdesi bir WebhookComment nesnesidir.

##### "Update" Olayının Yapısı
"update" olayının istek gövdesi bir WebhookComment nesnesidir.

##### "Delete" Olayının Yapısı
"delete" olayının istek gövdesi bir WebhookComment nesnesidir.

    14 Kasım 2023 itibarıyla değişiklik
    Daha önce "delete" olayının istek gövdesi yalnızca yorum kimliğini (id) içeriyordu. Artık silme anındaki yorumun tamamını içerir.


Her anahtar gövdede her zaman bulunur. Yorumun bir alan için değeri olmadığında gövde `null` taşır
(booleans için `false`, listeler için `[]`), böylece bir teslimatın şekli yorumdan yoruma asla değişmez.

[inline-code-attrs-start title = 'WebhookComment Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Yorumun kimliği (id). **/
    id: string
    /** Yorum iş parçacığını tanımlayan kimlik veya URL. Normalize edilmiştir. **/
    urlId: string
    /** Yorumun bırakıldığı yeri işaret eden URL. **/
    url: string | null
    /** Yorumu bırakan kullanıcının kimliği. SSO ise, kiracı (tenant) kimliği önekiyle gelir. **/
    userId: string | null
    /** Yorumu bırakan kullanıcının e-postası. **/
    commenterEmail: string | null
    /** Yorum widget'ında görünen kullanıcı adı. SSO ile, displayName olabilir. **/
    commenterName: string
    /** Ham yorum metni. **/
    comment: string
    /** Ayrıştırma sonrası yorum metni. **/
    commentHTML: string
    /** Yorumun harici kimliği. **/
    externalId: string | null
    /** Üst yorumun kimliği. **/
    parentId: string | null
    /** Yorumun bırakıldığı UTC tarihi. **/
    date: UTC_ISO_DateString
    /** Oyların birleşik karması (yukarı - aşağı). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Kullanıcı yorum yaparken oturum açtıysa, yorumu doğruladıysa veya yorum bırakıldığında oturumunu doğruladıysa true. **/
    verified: boolean
    /** Yorumun doğrulandığı UTC tarihi. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Bir moderatör yorumu incelenmiş olarak işaretlediyse. **/
    reviewed: boolean
    /** Avatarın konumu veya base64 kodlaması. Yalnızca SSO ile bu değer iletildiyse base64 olur. **/
    avatarSrc: string | null
    /** Yorum manuel veya otomatik olarak spam olarak işaretlendi mi? **/
    isSpam: boolean
    /** Yorum otomatik olarak spam olarak işaretlendi mi? **/
    aiDeterminedSpam: boolean
    /** Yorumda görseller var mı? **/
    hasImages: boolean
    /** "Most Relevant" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumber: number | null
    /** "Oldest First" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumberOF: number | null
    /** "Newest First" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumberNF: number | null
    /** Yorum otomatik mi yoksa manuel mi onaylandı? **/
    approved: boolean
    /** Yorum yazıldığında kullanıcının yerel ayar kodu (format: en_us). **/
    locale: string | null
    /** Yorumda yazılan ve başarıyla ayrıştırılan @mention'lar. Yoksa boştur. **/
    mentions: CommentUserMention[]
    /** Yorumun geldiği alan adı. **/
    domain: string | null
    /** Bu yorumla ilişkili moderasyon grubu kimlikleri. Yoksa boştur. **/
    moderationGroupIds: string[]
    /** Bu yorumda yorumcu adının yanında gösterilen rozetler, olayın oluşturulduğu andaki haliyle. Yoksa boştur. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Kullanıcılar bir yorumda etiketlendiğinde, bilgi `mentions` adlı bir listede saklanır. Bu listedeki her nesne
aşağıdaki yapıya sahiptir.

[inline-code-attrs-start title = 'Webhook Mentions Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Kullanıcı kimliği. SSO kullanıcıları için, kiracı (tenant) kimliğiniz önekiyle gelir. **/
    id: string
    /** @ simgesi dahil, nihai @mention etiket metni. **/
    tag: string
    /** @ simgesi dahil, orijinal @mention etiket metni. **/
    rawTag: string
    /** Etiketlenen kullanıcı türü. user = FastComments.com hesabı. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Kullanıcı bildirimlerden çıkış yapmış olsa bile bu yine de true olarak ayarlanır. **/
    sent: boolean
}
[inline-code-end]

`badges` listesindeki her girdi aşağıdaki yapıya sahiptir. Rozet türleri için [Rozetler rehberine](/guide-badges.html) bakın.

[inline-code-attrs-start title = 'Webhook Badge Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** Bu kullanıcıya verilen rozetin kimliği. **/
    id: string
    /** Rozet türü. **/
    type: number
    /** Rozetin üzerine gelindiğinde gösterilen rozet açıklaması. **/
    description: string
    /** Rozet üzerinde gösterilen metin. **/
    displayLabel?: string | null
    /** Rozet bir görsel kullanıyorsa, gösterilen görsel. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Rozete uygulanan ek bir CSS sınıfı. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP Metotları

Her webhook olay türü için HTTP metodunu yönetim panelinde yapılandırabilirsiniz:

- **Create Olayı**: POST veya PUT (varsayılan: PUT)
- **Update Olayı**: POST veya PUT (varsayılan: PUT)
- **Delete Olayı**: DELETE, POST veya PUT (varsayılan: DELETE)

Tüm istekler bir kimlik (ID) içerdiğinden, Create ve Update işlemleri varsayılan olarak idempotenttir (PUT). Aynı Create veya Update isteğini tekrarlamak, sizin tarafınızda yinelenen nesneler oluşturmamalıdır.

#### İstek Başlıkları

Her webhook isteği aşağıdaki başlıkları içerir:

| Başlık | Açıklama |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | API Gizli Anahtarınız (API Secret) |
| `X-FastComments-Timestamp` | İsteğin imzalandığı Unix zaman damgası (saniye) |
| `X-FastComments-Signature` | HMAC-SHA256 imzası (`sha256=<hex>`) |

HMAC imzasını doğrulama hakkında bilgi için [Güvenlik ve API Tokenları](/guide-webhooks.html#webhooks-api-tokens) sayfasına bakın.