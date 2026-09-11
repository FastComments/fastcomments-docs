Webhooks aracılığıyla gönderilen tek yapı, aşağıda TypeScript ile açıklanan WebhookComment nesnesidir.

#### WebhookComment Nesne Yapısı

##### "Create" Olayı Yapısı  
"create" olayı isteği gövdesi bir WebhookComment nesnesidir.

##### "Update" Olayı Yapısı  
"update" olayı isteği gövdesi bir WebhookComment nesnesidir.

##### "Delete" Olayı Yapısı  
"delete" olayı isteği gövdesi bir WebhookComment nesnesidir.

    Değişiklik 14 Kasım 2023 tarihinden itibaren
    Daha önce "delete" olayı isteği gövdesi yalnızca yorum kimliğini içeriyordu. Şimdi silme anındaki tam yorumu içeriyor.

Gövde içinde her anahtar her zaman bulunur. Yorum bir alan için değere sahip olmadığında gövde `null` (veya booleans için `false` ve listeler için `[]`) taşır, böylece teslimatın şekli bir yorumdan diğerine asla değişmez.

[inline-code-attrs-start title = 'WebhookComment Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Yorumun kimliği. **/
    id: string
    /** Yorum dizisini tanımlayan kimlik veya URL. Normalleştirilmiş. **/
    urlId: string
    /** Yorumun bırakıldığı yeri gösteren URL. **/
    url: string | null
    /** Yorumu bırakan kullanıcının kimliği. SSO ise, tenant kimliğiyle ön eklenir. **/
    userId: string | null
    /** Yorumu bırakan kullanıcının e-posta adresi. **/
    commenterEmail: string | null
    /** Yorum widget'ında gösterilen kullanıcının adı. SSO ile, displayName olabilir. **/
    commenterName: string
    /** Ham yorumun metni. **/
    comment: string
    /** Ayrıştırma sonrası yorum metni. **/
    commentHTML: string
    /** Yorumun dış kimliği. **/
    externalId: string | null
    /** Üst yorumun kimliği. **/
    parentId: string | null
    /** Yorumun bırakıldığı UTC tarih. **/
    date: UTC_ISO_DateString
    /** Oyların birleşik karması (yukarı - aşağı). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Kullanıcı yorum yaptığında oturum açmışsa, yorumu doğrulamışsa veya yorum bırakıldığında oturumu doğrulamışsa true. **/
    verified: boolean
    /** Yorumun doğrulandığı UTC tarih. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Bir moderatör yorumun incelendiğini işaretlediyse. **/
    reviewed: boolean
    /** Avatarın konumu veya base64 kodlaması. SSO ile gönderilen değer base64 ise sadece base64 olur. **/
    avatarSrc: string | null
    /** Yorum manuel veya otomatik olarak spam olarak işaretlendi mi? **/
    isSpam: boolean
    /** Yorum otomatik olarak spam olarak işaretlendi mi? **/
    aiDeterminedSpam: boolean
    /** Yorumda resimler var mı? **/
    hasImages: boolean
    /** "Most Relevant" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumber: number | null
    /** "Oldest First" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumberOF: number | null
    /** "Newest First" sıralama yönü için yorumun bulunduğu sayfa numarası. **/
    pageNumberNF: number | null
    /** Yorum otomatik veya manuel olarak onaylandı mı? **/
    approved: boolean
    /** Yorum yazıldığında kullanıcının yerel kodu (format: en_us). **/
    locale: string | null
    /** Yorumda yazılan ve başarıyla ayrıştırılan @mentions. Hiçbiri yoksa boş. **/
    mentions: CommentUserMention[]
    /** Yorumun geldiği domain. **/
    domain: string | null
    /** Bu yorumla ilişkili moderasyon grup kimlikleri. Hiçbiri yoksa boş. **/
    moderationGroupIds: string[]
}
[inline-code-end]

Kullanıcılar bir yorumda etiketlendiğinde, bilgi `mentions` adlı bir listede saklanır. Bu listedeki her nesnenin yapısı aşağıdaki gibidir.

[inline-code-attrs-start title = 'Webhook Mentions Nesnesi'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Kullanıcı kimliği. SSO kullanıcıları için tenant kimliği ön eklenir. **/
    id: string
    /** Son @mention etiket metni, @ sembolü dahil. **/
    tag: string
    /** Orijinal @mention etiket metni, @ sembolü dahil. **/
    rawTag: string
    /** Etiketlenen kullanıcının tipi. user = FastComments.com hesabı. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Kullanıcı bildirimlerden çıkmayı seçse bile, bu değer true olarak ayarlanır. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Yöntemleri

Her webhook olay türü için HTTP yöntemini yönetim panelinde yapılandırabilirsiniz:

- **Create Olayı**: POST veya PUT (varsayılan: PUT)
- **Update Olayı**: POST veya PUT (varsayılan: PUT)
- **Delete Olayı**: DELETE, POST veya PUT (varsayılan: DELETE)

Tüm istekler bir ID içerdiğinden, Create ve Update işlemleri varsayılan olarak (PUT) idempotenttir. Aynı Create veya Update isteğini tekrarlamak, tarafınızda yinelenen nesneler oluşturmaz.

#### İstek Başlıkları

Her webhook isteği aşağıdaki başlıkları içerir:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | API Gizliniz |
| `X-FastComments-Timestamp` | İsteğin imzalandığı Unix zaman damgası (saniye) |
| `X-FastComments-Signature` | HMAC-SHA256 imzası (`sha256=<hex>`) |

HMAC imzasını doğrulama hakkında bilgi için [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) sayfasına bakın.