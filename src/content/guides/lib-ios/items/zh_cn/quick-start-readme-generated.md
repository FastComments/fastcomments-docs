---
显示评论小部件的最小设置：

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

将 "demo" 替换为您的 FastComments 租户 ID。`urlId` 用于标识存储评论的页面或讨论线程。

---
---