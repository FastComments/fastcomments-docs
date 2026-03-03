FastComments kullanımı kolay bir SSO çözümü sağlar. HMAC tabanlı entegrasyon ile bir kullanıcının bilgilerini güncellemek, kullanıcının güncellenmiş payload ile sayfayı yüklemesini sağlamak kadar basittir.

Bununla birlikte, uygulamanızın tutarlılığını artırmak için bir kullanıcıyı bu akışın dışında yönetmek isteyebilirsiniz.

SSO User API, SSOUsers adını verdiğimiz nesneleri CRUD yapmanın bir yolunu sağlar. Bu nesneler normal Users'dan farklıdır ve tür güvenliği için ayrı tutulur.

The structure for the SSOUser object is as follows:

[inline-code-attrs-start title = 'SSOUser Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Yönetici izni - Bu işarete sahip SSO kullanıcıları SSO Yönetici olarak faturalandırılır (normal SSO kullanıcılarından ayrı olarak)
    isAdminAdmin?: boolean // Yönetici izni - Bu işarete sahip SSO kullanıcıları SSO Yönetici olarak faturalandırılır (normal SSO kullanıcılarından ayrı olarak)
    isCommentModeratorAdmin?: boolean // Moderatör izni - Bu işarete sahip SSO kullanıcıları SSO Moderatör olarak faturalandırılır (normal SSO kullanıcılarından ayrı olarak)
    /** Eğer null ise, Erişim Kontrolü kullanıcıya uygulanmaz. Eğer boş bir liste ise, bu kullanıcı herhangi bir sayfayı göremeyecek ve diğer kullanıcıları @mention yapamayacaktır. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Diğer kullanıcıların bu kullanıcının profilindeki etkinliklerini (yorumlar dahil) görmesine izin verme. Varsayılan olarak güvenli profiller sağlamak için true'dur. **/
    isProfileActivityPrivate?: boolean
    /** Diğer kullanıcıların bu kullanıcının profiline yorum bırakmasını veya mevcut profil yorumlarını görmesini engelleyin. Varsayılan değer false'tur. **/
    isProfileCommentsPrivate?: boolean
    /** Diğer kullanıcıların bu kullanıcıya doğrudan mesaj göndermesine izin verme. Varsayılan değer false'tur. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Kullanıcı rozetleri için isteğe bağlı yapılandırma. **/
    badgeConfig?: {
        /** Kullanıcıya atamak için rozet ID'lerinden oluşan dizi. 30 rozet ile sınırlıdır. Sıra korunur. Bunlar tüm sayfalarda görünen genel rozetlerdir. **/
        badgeIds: string[]
        /** Geçerli sayfaya (urlId) özgü rozet ID'lerinden oluşan isteğe bağlı dizi. Bu rozetler yalnızca atandıkları sayfada gösterilir. **/
        pageBadgeIds?: string[]
        /** Eğer true ise, mevcut görüntülenen tüm rozetleri sağlananlarla değiştirir. Genel ve sayfa-özel rozetler bağımsız olarak geçersiz kılınır. Eğer false ise, mevcut rozetlere ekler. **/
        override?: boolean
        /** Eğer true ise, kullanıcı giriş yaptığında rozet görüntüleme özelliklerini kiracı yapılandırmasından günceller. **/
        update?: boolean
    }
}
[inline-code-end]

### Billing for SSO Users

SSO kullanıcıları izin bayraklarına göre farklı şekilde faturalandırılır:

- **Regular SSO Users**: Yönetici veya moderatör izni olmayan kullanıcılar normal SSO kullanıcıları olarak faturalandırılır
- **SSO Admins**: `isAccountOwner` veya `isAdminAdmin` bayraklarına sahip kullanıcılar ayrı olarak SSO Admin olarak faturalandırılır (normal kiracı yöneticileri ile aynı oran)
- **SSO Moderators**: `isCommentModeratorAdmin` bayrağına sahip kullanıcılar ayrı olarak SSO Moderatör olarak faturalandırılır (normal moderatörlerle aynı oran)

**Important**: Çifte faturalandırmayı önlemek için sistem, SSO kullanıcılarını e-posta adresine göre normal kiracı kullanıcıları ve moderatörlerle otomatik olarak deduplikasyon yapar. Bir SSO kullanıcısının e-postası bir normal kiracı kullanıcısı veya moderatör ile aynıysa, iki kez faturalandırılmazlar.

### Access Control

Kullanıcılar gruplara ayrılabilir. Bunun için `groupIds` alanı vardır ve isteğe bağlıdır.

### @Mentions

Varsayılan olarak `@mentions`, `@` karakteri yazıldığında diğer sso kullanıcılarını aramak için `username`'i kullanır. Eğer `displayName` kullanılırsa, `displayName` için bir eşleşme olduğunda `username` ile eşleşen sonuçlar göz ardı edilir ve `@mention` arama sonuçları `displayName`'i kullanır.

### Subscriptions

FastComments ile kullanıcılar, yorum bileşenindeki zil simgesine tıklayıp Abone Ol'a tıklayarak bir sayfaya abone olabilirler.

Normal bir kullanıcı ile bildirim ayarlarına göre onlara bildirim e-postaları göndeririz.

SSO Kullanıcıları ile geriye dönük uyumluluk için bunu böleriz. Kullanıcılar yalnızca `optedInSubscriptionNotifications`'ı `true` olarak ayarlarsanız ek abonelik bildirim e-postaları gönderilecektir.

### Badges

SSO kullanıcılara `badgeConfig` özelliğini kullanarak rozet atayabilirsiniz. Rozetler, yorumlarda kullanıcının adı yanında görünen görsel göstergelerdir.

- `badgeIds` - Kullanıcıya atamak için rozet ID'lerinden oluşan dizi. Bunlar tüm sayfalarda görünen genel rozetlerdir. FastComments hesabınızda oluşturulmuş geçerli rozet ID'leri olmalıdır. 30 rozet ile sınırlıdır.
- `pageBadgeIds` - Geçerli sayfaya (`urlId`) özgü isteğe bağlı rozet ID'leri dizisi. Bu rozetler yalnızca atandıkları sayfada gösterilir. Farklı sayfalar aynı kullanıcı için farklı sayfa-özel rozetlere sahip olabilir.
- `override` - Eğer true ise, mevcut görüntülenen tüm rozetler sağlananlarla değiştirilecektir. Genel ve sayfa-özel rozetler bağımsız olarak geçersiz kılınır — genel rozetleri geçersiz kılmak sayfa-özel rozetleri etkilemez ve tersi de geçerlidir. Eğer false veya atlanırsa, sağlanan rozetler mevcut rozetlere eklenir.
- `update` - Eğer true ise, kullanıcı giriş yaptığında rozet görüntüleme özellikleri kiracı yapılandırmasından güncellenecektir.