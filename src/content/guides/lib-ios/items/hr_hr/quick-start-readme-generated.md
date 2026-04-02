---
Minimalna konfiguracija za prikaz widgeta za komentare:

```swift
import SwiftUI
import FastCommentsUI

struct ContentView: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "demo",
            urlId: "my-page-1",
            url: "https://example.com/page-1",
            pageTitle: "My Page"
        )
    )

    var body: some View {
        FastCommentsView(sdk: sdk)
            .task {
                try? await sdk.load()
            }
    }
}
```

Zamijenite `"demo"` sa svojim FastComments tenant ID-jem. `urlId` identificira stranicu ili nit na kojoj se komentari pohranjuju.

---
---