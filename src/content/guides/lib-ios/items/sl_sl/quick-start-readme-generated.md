Najmanjša nastavitev za prikaz pripomočka za komentarje:

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

Zamenjajte `"demo"` z vašim FastComments tenant ID. `urlId` identificira stran ali nit, kjer so komentarji shranjeni.

---
---