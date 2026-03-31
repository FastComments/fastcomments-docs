FastComments podržava tri načina autentikacije:

1. **Anonimno** -- bez SSO tokena; korisnici dobivaju identitete temeljene na sesiji
2. **Jednostavni SSO** -- klijentski token za demo i testiranje (nije sigurno)
3. **Sigurni SSO** -- token potpisan na serveru za produkciju

### Jednostavni SSO

Koristan za demo i lokalno testiranje. Bilo tko može se predstaviti kao bilo koji korisnik pomoću Jednostavnog SSO-a, stoga ga nemojte koristiti u produkciji.

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

`SimpleSSOUserData` također podržava opcionalna polja:

- `id` -- ID korisnika (zadano na email ako nije postavljeno)
- `displayName` -- odvojeno prikazano ime
- `displayLabel` -- prilagođena oznaka prikazana pored imena (npr. "VIP")
- `websiteUrl` -- poveznica na korisničko ime
- `locale` -- kod lokalizacije
- `isProfileActivityPrivate` -- sakrij aktivnost profila (zadano na true)

### Sigurni SSO

U produkciji, vaš backend generira potpisani SSO token koristeći vaš API secret. iOS aplikacija dohvaća taj token s vašeg servera i prosljeđuje ga u konfiguraciju.

**Na vašem backendu** (koristeći FastComments Swift SDK ili bilo koji jezik):

```swift
let userData = SecureSSOUserData(
    id: "user-123",
    email: "user@example.com",
    username: "Display Name",
    avatar: "https://example.com/avatar.jpg"
)
let sso = try FastCommentsSSO.createSecure(apiKey: "YOUR_API_KEY", secureSSOUserData: userData)
let token = try sso.prepareToSend()
// Vratite ovaj token svojoj iOS aplikaciji putem vašeg API-ja
```

**U vašoj iOS aplikaciji:**

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
            // Dohvatite token s vašeg backenda
            let token = try? await fetchSSOTokenFromYourBackend()
            // Kreirajte novu konfiguraciju s tokenom, ili ga postavite prije učitavanja
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` podržava dodatna polja:

- `optedInNotifications` -- pristanak na e-mail obavijesti
- `displayLabel` -- prilagođena oznaka
- `displayName` -- prikazano ime
- `websiteUrl` -- URL web stranice
- `groupIds` -- članstva u grupama
- `isAdmin` -- administratorske privilegije
- `isModerator` -- moderatorske privilegije
- `isProfileActivityPrivate` -- privatnost profila

---
---