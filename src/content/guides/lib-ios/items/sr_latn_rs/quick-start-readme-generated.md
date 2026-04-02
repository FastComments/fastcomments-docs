Minimalna podešavanja za prikaz komentarskog widgeta:

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

Zamenite `"demo"` sa vašim FastComments tenant ID. Polje `urlId` identifikuje stranicu ili nit gde se komentari čuvaju.