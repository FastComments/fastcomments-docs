FastComments podpira tri načine overjanja:

1. **Anonimno** -- brez SSO žetona; uporabniki dobijo identitete na podlagi seje
2. **Preprost SSO** -- odjemalski žeton za predstavitve in testiranje (ni varen)
3. **Varen SSO** -- strežniško podpisan žeton za produkcijo

### Preprost SSO

Uporaben za predstavitve in lokalno testiranje. S preprostim SSO si lahko kdorkoli izda za katerega koli uporabnika, zato ga ne uporabljajte v produkciji.

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

`SimpleSSOUserData` podpira tudi neobvezna polja:

- `id` -- ID uporabnika (privzeto e-pošta, če ni nastavljen)
- `displayName` -- ločeno prikazano ime
- `displayLabel` -- oznaka po meri, prikazana poleg imena (npr. "VIP")
- `websiteUrl` -- povezava na imenu uporabnika
- `locale` -- koda jezikovne okolice
- `isProfileActivityPrivate` -- skrij aktivnost profila (privzeto true)

### Varen SSO

V produkciji vaš zaledni sistem ustvari podpisan SSO žeton z vašo API skrivnostjo. iOS aplikacija pridobi ta žeton z vašega strežnika in ga posreduje v konfiguracijo.

**Na vašem zalednem sistemu** (z uporabo FastComments Swift SDK ali katerega koli jezika):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Vrnite ta žeton svoji iOS aplikaciji prek vašega API-ja
```

**V vaši iOS aplikaciji:**

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
            // Pridobite žeton z vašega zalednega sistema
            let token = try? await fetchSSOTokenFromYourBackend()
            // Ustvarite novo konfiguracijo z žetonom ali ga nastavite pred nalaganjem
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` podpira dodatna polja:

- `optedInNotifications` -- prijava na e-poštna obvestila
- `displayLabel` -- oznaka po meri
- `displayName` -- prikazano ime
- `websiteUrl` -- URL spletnega mesta
- `groupIds` -- članstva v skupinah
- `isAdmin` -- skrbniške pravice
- `isModerator` -- moderatorske pravice
- `isProfileActivityPrivate` -- zasebnost profila

---
---