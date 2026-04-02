Minimalna konfiguracja do wyświetlenia widżetu komentarzy:

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

Zastąp "demo" swoim identyfikatorem najemcy FastComments. `urlId` identyfikuje stronę lub wątek, w którym przechowywane są komentarze.