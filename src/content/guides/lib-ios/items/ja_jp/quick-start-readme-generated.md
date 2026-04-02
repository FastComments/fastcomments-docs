コメントウィジェットを表示するための最小限のセットアップ:

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

"demo" をあなたの FastComments テナント ID に置き換えてください。`urlId` はコメントが保存されるページまたはスレッドを識別します。

---
---