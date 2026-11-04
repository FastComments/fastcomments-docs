FastComments, kullanımı kolay bir SSO çözümü sunar. HMAC tabanlı entegrasyonla bir kullanıcının bilgilerini güncellemek, kullanıcının güncellenmiş bir payload ile sayfayı yüklemesi kadar basittir.

Ancak, uygulamanızın tutarlılığını artırmak için bir kullanıcıyı bu akışın dışında yönetmek istenebilir.

SSO User API, SSOUsers olarak adlandırdığımız nesneleri CRUD yapmanın bir yolunu sağlar. Bu nesneler normal Users nesnelerinden farklıdır ve tip güvenliği için ayrı tutulur.

SSOUser nesnesinin yapısı aşağıdaki gibidir:

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
    isAccountOwner?: boolean // Admin izni - Bu bayrağa sahip SSO kullanıcıları SSO Admin olarak faturalandırılır (normal SSO kullanıcılardan ayrı)
    isAdminAdmin?: boolean // Admin izni - Bu bayrağa sahip SSO kullanıcıları SSO Admin olarak faturalandırılır (normal SSO kullanıcılardan ayrı)
    isCommentModeratorAdmin?: boolean // Moderatör izni - Bu bayrağa sahip SSO kullanıcıları SSO Moderatör olarak faturalandırılır (normal SSO kullanıcılardan ayrı)
    /** If null, Access Control will not be applied to the user. If an empty list, this user will not be able to see any pages or @mention other users. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Don't let other users see this user's activity, including comments, on their profile. Default is true to provide secure profiles by default. **/
    isProfileActivityPrivate?: boolean
    /** Don't let other users leave comments on the user's profile, or see existing profile comments. Default false. **/
    isProfileCommentsPrivate?: boolean
    /** Don't let other users send direct messages to this user. Default false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of badge IDs to assign to the user. Limited to 30 badges. Order is respected. These are global badges visible on all pages. **/
        badgeIds: string[]
        /** Array of badge IDs scoped to the current page (urlId). These badges are only displayed on the page where they were assigned. **/
        pageBadgeIds?: string[]
        /** If true, replaces all existing displayed badges with the provided ones. Global and page-scoped badges are overridden independently. If false, adds to existing badges. **/
        override?: boolean
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean
    }
}
[inline-code-end]

### SSO Kullanıcılarının Faturalandırılması

SSO kullanıcıları, izin bayraklarına göre farklı şekilde faturalandırılır:

- **Normal SSO Kullanıcıları**: Admin veya moderatör izinlerine sahip olmayan kullanıcılar normal SSO kullanıcıları olarak faturalandırılır
- **SSO Adminler**: `isAccountOwner` veya `isAdminAdmin` bayraklarına sahip kullanıcılar ayrı olarak SSO Admin olarak faturalandırılır (normal kiracı adminleriyle aynı ücret)
- **SSO Moderatörler**: `isCommentModeratorAdmin` bayrağına sahip kullanıcılar ayrı olarak SSO Moderatör olarak faturalandırılır (normal moderatörlerle aynı ücret)

**Önemli**: Çift faturalandırmayı önlemek için sistem, SSO kullanıcılarını e-posta adresine göre normal kiracı kullanıcıları ve moderatörlerle otomatik olarak tekilleştirir. Bir SSO kullanıcısı, normal bir kiracı kullanıcısı veya moderatörle aynı e-postaya sahipse, iki kez faturalandırılmaz.

### Erişim Kontrolü

Kullanıcılar gruplara ayrılabilir. `groupIds` alanı bu amaçla kullanılır ve isteğe bağlıdır.

### @Mentions

Varsayılan olarak `@mentions`, `@` karakteri yazıldığında diğer SSO kullanıcılarını aramak için `username` alanını kullanır. `displayName` kullanılıyorsa, `displayName` için bir eşleşme olduğunda `username` ile eşleşen sonuçlar yok sayılır ve `@mention` arama sonuçları `displayName` alanını kullanır.

### Abonelikler

FastComments ile kullanıcılar, yorum widget'ındaki zil simgesine tıklayarak ve Subscribe'a tıklayarak bir sayfaya abone olabilir.

Normal bir kullanıcıda, bildirim ayarlarına göre onlara bildirim e-postaları göndeririz.

SSO Kullanıcılarında, geriye dönük uyumluluk için bunu ayırıyoruz. Kullanıcılara bu ek abonelik bildirim e-postaları yalnızca `optedInSubscriptionNotifications` değerini `true` olarak ayarlarsanız gönderilir.

### Rozetler

`badgeConfig` özelliğini kullanarak SSO kullanıcılara rozet atayabilirsiniz. Rozetler, yorumlarda kullanıcının adının yanında görünen görsel göstergelerdir.

- `badgeIds` - Kullanıcıya atanacak rozet kimlikleri dizisi. Bunlar tüm sayfalarda görünen global rozetlerdir. FastComments hesabınızda oluşturulmuş geçerli rozet kimlikleri olmalıdır. 30 rozetle sınırlıdır.
- `pageBadgeIds` - Geçerli sayfaya (`urlId`) özel rozet kimliklerinin isteğe bağlı dizisi. Bu rozetler yalnızca atandıkları sayfada görüntülenir. Farklı sayfalar, aynı kullanıcı için farklı sayfaya özel rozetlere sahip olabilir.
- `override` - True ise, mevcut tüm görüntülenen rozetler sağlananlarla değiştirilir. Global ve sayfaya özel rozetler bağımsız olarak geçersiz kılınır — global rozetleri geçersiz kılmak sayfaya özel rozetleri etkilemez, tersi de geçerlidir. False veya belirtilmemişse, sağlanan rozetler mevcut rozetlere eklenir.
- `update` - True ise, kullanıcı her giriş yaptığında rozet görüntüleme özellikleri kiracı yapılandırmasından güncellenir.