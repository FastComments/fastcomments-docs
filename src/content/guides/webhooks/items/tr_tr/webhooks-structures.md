Webhook'lar aracılığıyla gönderilen tek yapı aşağıda TypeScript olarak özetlenen WebhookComment nesnesidir.

#### WebhookComment Nesnesi Yapısı

##### "Create" Olay Yapısı
"create" olayının istek gövdesi bir WebhookComment nesnesidir.

##### "Update" Olay Yapısı
"update" olayının istek gövdesi bir WebhookComment nesnesidir.

##### "Delete" Olay Yapısı
"delete" olayının istek gövdesi bir WebhookComment nesnesidir.

    14 Kasım 2023 tarihi itibarıyla değişiklik
    Önceden "delete" olayının istek gövdesi yalnızca yorum id'sini içeriyordu. Artık silinme anındaki tam yorumu içeriyor.


[inline-code-attrs-start title = 'WebhookComment Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Yorumun id'si. **/
    id: string
    /** Yorum dizisini tanımlayan id veya URL. Normalleştirilmiş. **/
    urlId: string
    /** Yorumun bırakıldığı yere işaret eden URL. **/
    url?: string
    /** Yorumu bırakan kullanıcı id'si. SSO ise tenant id ile öneklenmiş olur. **/
    userId?: string
    /** Yorumu bırakan kullanıcının e-postası. **/
    commenterEmail?: string
    /** Yorum bileşeninde görünen kullanıcı adı. SSO ile displayName olabilir. **/
    commenterName: string
    /** Ham yorum metni. **/
    comment: string
    /** Ayrıştırıldıktan sonra yorum metni. **/
    commentHTML: string
    /** Yorumun harici id'si. **/
    externalId?: string
    /** Üst yorumun id'si. **/
    parentId?: string | null
    /** Yorumun bırakıldığı UTC tarihi. **/
    date: UTC_ISO_DateString
    /** Oyların birleşik karması (yukarı - aşağı). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Kullanıcı yorumu yazarken giriş yaptıysa, yorumu doğruladıysa veya yorum bırakılırken oturumunu doğruladıysa true. **/
    verified: boolean
    /** Yorumun doğrulandığı tarih. **/
    verifiedDate?: number
    /** Moderatörün yorumu incelendi olarak işaretleyip işaretlemediği. **/
    reviewed: boolean
    /** Avatarın konumu veya base64 kodlaması. Sadece SSO ile base64 olarak gönderildiyse base64 olacaktır. **/
    avatarSrc?: string
    /** Yorum manuel mi yoksa otomatik olarak spam olarak mı işaretlendi? **/
    isSpam: boolean
    /** Yorum otomatik olarak spam olarak mı işaretlendi? **/
    aiDeterminedSpam: boolean
    /** Yorumda resimler var mı? **/
    hasImages: boolean
    /** Yorumun "En İlgili" sıralama yönü için bulunduğu sayfa numarası. **/
    pageNumber: number
    /** Yorumun "En Eski İlk" sıralama yönü için bulunduğu sayfa numarası. **/
    pageNumberOF: number
    /** Yorumun "En Yeni İlk" sıralama yönü için bulunduğu sayfa numarası. **/
    pageNumberNF: number
    /** Yorum otomatik olarak mı yoksa manuel olarak mı onaylandı? **/
    approved: boolean
    /** Yorum yazıldığında kullanıcının yerel kodu (format: en_us). **/
    locale: string
    /** Yorumda yazılan ve başarıyla ayrıştırılan @mention'lar. **/
    mentions?: CommentUserMention[]
    /** Yorumun geldiği alan adı. **/
    domain?: string
    /** Bu yorumla ilişkili isteğe bağlı moderasyon grup id'lerinin listesi. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Kullanıcılar bir yorumda etiketlendiğinde, bilgiler `mentions` adlı bir listede saklanır. Bu listedeki her nesnenin yapısı aşağıdaki gibidir.

[inline-code-attrs-start title = 'Webhook Mention Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Kullanıcı id'si. SSO kullanıcıları için tenant id ile öneklenmiş olacaktır. **/
    id: string
    /** Son @mention etiket metni, @ sembolü dahil. **/
    tag: string
    /** Orijinal @mention etiket metni, @ sembolü dahil. **/
    rawTag: string
    /** Hangi tür kullanıcı etiketlendi. user = FastComments.com hesabı. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Kullanıcı bildirimlerden çıkmış olsa bile bu yine de true olarak ayarlanır. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Yöntemleri

Her webhook olay türü için HTTP metodunu yönetici panelinden yapılandırabilirsiniz:

- **Oluşturma Olayı**: POST veya PUT (varsayılan: PUT)
- **Güncelleme Olayı**: POST veya PUT (varsayılan: PUT)
- **Silme Olayı**: DELETE, POST veya PUT (varsayılan: DELETE)

Tüm istekler bir ID içerdiğinden, Create ve Update işlemleri varsayılan olarak idempotenttir (PUT). Aynı Create veya Update isteğinin tekrarlanması, tarafınızda çift nesneler oluşturmamalıdır.

#### İstek Başlıkları

Her webhook isteği aşağıdaki başlıkları içerir:

| Başlık | Açıklama |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | API Gizli Anahtarınız |
| `X-FastComments-Timestamp` | İsteğin imzalandığı Unix zaman damgası (saniye) |
| `X-FastComments-Signature` | HMAC-SHA256 imzası (`sha256=<hex>`) |

HMAC imzasını doğrulama hakkında bilgi için [Güvenlik ve API Jetonları](/guide-webhooks.html#webhooks-api-tokens) sayfasına bakın.