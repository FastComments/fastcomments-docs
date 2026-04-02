ההגדרה המינימלית להצגת ווידג'ט תגובות:

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

החלף את `"demo"` ב־tenant ID של FastComments שלך. ה־`urlId` מזהה את העמוד או השרשור שבו מאוחסנות התגובות.

---
---