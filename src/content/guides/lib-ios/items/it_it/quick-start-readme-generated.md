---
La configurazione minima per visualizzare un widget di commenti:

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

Sostituisci "demo" con il tuo ID tenant FastComments. Il `urlId` identifica la pagina o il thread in cui vengono memorizzati i commenti.

---
---