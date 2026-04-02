FastComments podržava tri načina autentifikacije:

1. **Anonymous** -- nema SSO tokena; korisnici dobijaju identitete zasnovane na sesiji
2. **Simple SSO** -- token na strani klijenta za demo i testiranje (nije sigurno)
3. **Secure SSO** -- token potpisan na serveru za produkciju

### Simple SSO

Koristan za demo i lokalno testiranje. Bilo ko može da se predstavi kao bilo koji korisnik pomoću Simple SSO, zato ga nemojte koristiti u produkciji.

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

`SimpleSSOUserData` takođe podržava opciona polja:

- `id` -- ID korisnika (podrazumevano email ako nije podešeno)
- `displayName` -- posebno prikazano ime
- `displayLabel` -- prilagođena oznaka prikazana pored imena (npr. "VIP")
- `websiteUrl` -- link na imenu korisnika
- `locale` -- jezički kod
- `isProfileActivityPrivate` -- sakrij aktivnost profila (podrazumevano je true)

### Secure SSO

U produkciji, vaš backend generiše potpisani SSO token koristeći vaš API secret. iOS aplikacija preuzima ovaj token sa vašeg servera i prosleđuje ga u konfiguraciju.

**On your backend** (using the FastComments Swift SDK or any language):

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

**In your iOS app:**

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
            // Preuzmite token sa vašeg backenda
            let token = try? await fetchSSOTokenFromYourBackend()
            // Kreirajte novu konfiguraciju sa tokenom, ili je postavite pre učitavanja
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` podržava dodatna polja:

- `optedInNotifications` -- prijava za email notifikacije
- `displayLabel` -- prilagođena oznaka
- `displayName` -- prikazano ime
- `websiteUrl` -- URL vebsajta
- `groupIds` -- članstva u grupama
- `isAdmin` -- adminske privilegije
- `isModerator` -- moderatorske privilegije
- `isProfileActivityPrivate` -- privatnost aktivnosti profila

---
---