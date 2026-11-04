FastComments podržava tri režima autentifikacije:

1. **Anonimni** -- bez SSO tokena; korisnici dobijaju identitete zasnovane na sesiji
2. **Jednostavni SSO** -- token na klijentskoj strani za demonstracije i testiranje (nije siguran)
3. **Sigurni SSO** -- token potpisan na serveru za produkciju

### Jednostavni SSO

Korisno za demonstracije i lokalno testiranje. Bilo ko može da se predstavi kao bilo koji korisnik pomoću Jednostavnog SSO-a, pa ga ne koristite u produkciji.

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

`SimpleSSOUserData` takođe podržava opcione polja:

- `id` -- ID korisnika (podrazumevano email ako nije postavljen)
- `displayName` -- odvojeni ime za prikaz
- `displayLabel` -- prilagođena oznaka prikazana pored imena (npr. "VIP")
- `websiteUrl` -- link na korisničkom imenu
- `locale` -- kod lokalizacije
- `isProfileActivityPrivate` -- sakrij aktivnost profila (podrazumevano true)

### Sigurni SSO

U produkciji, vaš backend generiše potpisani SSO token koristeći vašu API tajnu. iOS aplikacija preuzima ovaj token sa vašeg servera i prosleđuje ga u konfiguraciju.

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
// Vratite ovu token vašoj iOS aplikaciji putem vašeg API-ja
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
            // Preuzmite token sa vašeg backend-a
            let token = try? await fetchSSOTokenFromYourBackend()
            // Kreirajte novu konfiguraciju sa tokenom, ili ga postavite pre učitavanja
            isLoadingToken = false
            try? await sdk.load()
        }
    }
}
```

`SecureSSOUserData` podržava dodatna polja:

- `optedInNotifications` -- prijava za email obaveštenja
- `displayLabel` -- prilagođena oznaka
- `displayName` -- ime za prikaz
- `websiteUrl` -- URL veb-sajta
- `groupIds` -- članstva u grupama
- `isAdmin` -- administratorske privilegije
- `isModerator` -- moderatorske privilegije
- `isProfileActivityPrivate` -- privatnost profila

---
---