顯示評論小工具的最小設定：

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

將 `"demo"` 替換為您的 FastComments 租戶 ID。`urlId` 用來識別儲存評論的頁面或討論串。