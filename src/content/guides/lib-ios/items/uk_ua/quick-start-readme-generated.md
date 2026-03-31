Мінімальні налаштування для відображення віджета коментарів:

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

Замініть `"demo"` на свій ідентифікатор орендаря FastComments. Параметр `urlId` ідентифікує сторінку або тему, де зберігаються коментарі.

---
---