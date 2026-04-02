Die minimale Einrichtung, um ein Kommentar-Widget anzuzeigen:

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

Ersetzen Sie `"demo"` durch Ihre FastComments Tenant-ID. Die `urlId` identifiziert die Seite oder den Thread, in dem die Kommentare gespeichert werden.

---
---