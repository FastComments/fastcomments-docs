FastComments üç kimlik doğrulama modunu destekler:

1. **Anonim** -- SSO belirteci yok; kullanıcılar oturum tabanlı kimlikler alır
2. **Basit SSO** -- demolar ve testler için istemci tarafı token (güvenli değildir)
3. **Güvenli SSO** -- üretim için sunucu tarafından imzalanmış token

### Basit SSO

Demolar ve yerel testler için faydalıdır. Basit SSO ile herkes herhangi bir kullanıcıyı taklit edebilir, bu yüzden üretimde kullanmayın.

```swift
import FastCommentsSwift

let userData = SimpleSSOUserData(
    username: "Jane Doe",
    email: "jane@example.com",
    avatar: "https://example.com/avatar.jpg"
)
let sso = FastCommentsSSO.createSimple(simpleSSOUserData: userData)
let token = try? sso.prepareToSend()

let config = FastCommentsWidgetConfig(
    tenantId: "YOUR_TENANT_ID",
    urlId: "my-page-1",
    sso: token
)
let sdk = FastCommentsSDK(config: config)
```

`SimpleSSOUserData` also supports optional fields:

- `id` -- kullanıcı kimliği (ayarlanmazsa varsayılan olarak e-posta)
- `displayName` -- ayrı görüntüleme adı
- `displayLabel` -- ismin yanına gösterilen özel etiket (ör. "VIP")
- `websiteUrl` -- kullanıcının adına verilen bağlantı
- `locale` -- yerel ayar kodu
- `isProfileActivityPrivate` -- profil etkinliğini gizle (varsayılan olarak true)

### Güvenli SSO

Üretimde, backend'iniz API gizli anahtarınızı kullanarak imzalanmış bir SSO tokeni oluşturur. iOS uygulaması bu tokeni sunucunuzdan alır ve config'e geçirir.

**Sunucunuzda** (FastComments Swift SDK veya herhangi bir dil kullanılarak):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Return this token to your iOS app via your API
```

**iOS uygulamanızda:**

```swift
struct MyView: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-page-1"
        )
    )
    @State private var isLoadingToken = true

    var body: some View {
        Group {
            if isLoadingToken {
                ProgressView("Loading...")
            } else {
                FastCommentsView(sdk: sdk)
            }
        }
        .task {
            // Tokeni sunucunuzdan al
            let token = try? await fetchSSOTokenFromYourBackend()
            // Token ile yeni bir config oluşturun veya yüklemeden önce ayarlayın
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` supports additional fields:

- `optedInNotifications` -- e-posta bildirimleri için onay
- `displayLabel` -- özel etiket
- `displayName` -- görünen ad
- `websiteUrl` -- web sitesi URL'si
- `groupIds` -- grup üyelikleri
- `isAdmin` -- yönetici ayrıcalıkları
- `isModerator` -- moderator ayrıcalıkları
- `isProfileActivityPrivate` -- profil gizliliği

---
---