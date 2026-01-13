Webhook'lar aracılığıyla gönderilen tek yapı aşağıda TypeScript ile özetlenen WebhookComment nesnesidir.

#### WebhookComment Nesnesi Yapısı

##### "Create" Olay Yapısı
"create" olay isteği gövdesi bir WebhookComment nesnesidir.

##### "Update" Olay Yapısı
"update" olay isteği gövdesi bir WebhookComment nesnesidir.

##### "Delete" Olay Yapısı
"delete" olay isteği gövdesi bir WebhookComment nesnesidir.

    14 Kasım 2023 itibarıyla değişiklik
    Daha önce "delete" olay isteği gövdesi yalnızca yorum id'sini içeriyordu. Artık silme anındaki tam yorumu içerir.


[inline-code-attrs-start title = 'WebhookComment Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Yorumun id'si. **/
    id: string
    /** Yorum dizisini tanımlayan id veya URL. Normalleştirilmiş. **/
    urlId: string
    /** Yorumun bırakıldığı yeri gösteren URL. **/
    url?: string
    /** Yorumu bırakan kullanıcı id'si. SSO ise, tenant id ile öneklenmiş. **/
    userId?: string
    /** Yorumu bırakan kullanıcının e-postası. **/
    commenterEmail?: string
    /** Yorum widget'ında görünen kullanıcı adı. SSO ile displayName olabilir. **/
    commenterName: string
    /** Ham yorum metni. **/
    comment: string
    /** Ayrıştırıldıktan sonraki yorum metni. **/
    commentHTML: string
    /** Yorumun harici id'si. **/
    externalId?: string
    /** Üst yorumun id'si. **/
    parentId?: string | null
    /** Yorumun bırakıldığı UTC tarih. **/
    date: UTC_ISO_DateString
    /** Oyların (yukarı - aşağı) toplamı. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Kullanıcı yorum yaparken giriş yapmışsa, yorumu doğrulanmışsa veya yorum bırakılırken oturum doğrulaması yapılmışsa true. **/
    verified: boolean
    /** Yorumun doğrulandığı tarih. **/
    verifiedDate?: number
    /** Bir moderatör yorum'u incelendi olarak işaretlediyse. **/
    reviewed: boolean
    /** Avatarın konumu veya base64 kodlaması. Yalnızca SSO ile base64 olarak gönderildiyse base64 olur. **/
    avatarSrc?: string
    /** Yorum manuel veya otomatik olarak spam olarak işaretlendi mi? **/
    isSpam: boolean
    /** Yorum otomatik olarak spam olarak işaretlendi mi? **/
    aiDeterminedSpam: boolean
    /** Yorumda görseller var mı? **/
    hasImages: boolean
    /** "Most Relevant" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumber: number
    /** "Oldest First" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumberOF: number
    /** "Newest First" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumberNF: number
    /** Yorum otomatik veya manuel olarak onaylandı mı? **/
    approved: boolean
    /** Yorum yazılırken kullanıcının yerel ayar kodu (format: en_us). **/
    locale: string
    /** Yoruma yazılan ve başarıyla ayrıştırılan @mention'lar. **/
    mentions?: CommentUserMention[]
    /** Yorumun geldiği alan adı. **/
    domain?: string
    /** Bu yorumla ilişkili isteğe bağlı moderasyon grup id'lerinin listesi. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Kullanıcılar bir yorumda etiketlendiğinde, bilgiler `mentions` adlı bir listede saklanır. O listedeki her nesne aşağıdaki yapıya sahiptir.

[inline-code-attrs-start title = 'Webhook Mentions Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Kullanıcı id'si. SSO kullanıcıları için, tenant id öneklenmiş olacaktır. **/
    id: string
    /** @ sembolü dahil son @mention etiket metni. **/
    tag: string
    /** @ sembolü dahil orijinal @mention etiket metni. **/
    rawTag: string
    /** Hangi tip kullanıcının etiketlendiği. user = FastComments.com hesabı. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Kullanıcı bildirimleri kapatmış olsa bile, bu yine de true olarak ayarlanır. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Yöntemleri

Yönetici panelinde her webhook olay türü için HTTP yöntemini yapılandırabilirsiniz:

- **Create Event**: POST veya PUT (varsayılan: PUT)
- **Update Event**: POST veya PUT (varsayılan: PUT)
- **Delete Event**: DELETE, POST veya PUT (varsayılan: DELETE)

Tüm istekler bir ID içerdiğinden, Create ve Update işlemleri varsayılan olarak idempotenttir (PUT). Aynı Create veya Update isteğini tekrarlamak, sizin tarafınızda yinelenen nesneler oluşturmemelidir.

#### İstek Başlıkları

Her webhook isteği aşağıdaki başlıkları içerir:

| Başlık | Açıklama |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | API Gizli Anahtarınız |
| `X-FastComments-Timestamp` | İsteğin imzalandığı Unix zaman damgası (saniye) |
| `X-FastComments-Signature` | HMAC-SHA256 imzası (`sha256=<hex>`) |

HMAC imzasını doğrulama hakkında bilgi için [Güvenlik ve API Anahtarları](/guides/webhooks/webhooks-api-tokens) sayfasına bakın.

---