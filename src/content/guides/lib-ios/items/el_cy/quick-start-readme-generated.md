---
Η ελάχιστη ρύθμιση για εμφάνιση ενός widget σχολίων:

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

Αντικαταστήστε το "demo" με το tenant ID του FastComments. Το `urlId` προσδιορίζει τη σελίδα ή το νήμα όπου αποθηκεύονται τα σχόλια.

---
---