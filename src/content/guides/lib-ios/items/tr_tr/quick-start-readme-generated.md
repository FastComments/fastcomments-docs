Bir yorum widget'ı görüntülemek için gereken minimum kurulum:

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

`"demo"` değerini kendi FastComments tenant ID'nizle değiştirin. `urlId`, yorumların saklandığı sayfayı veya thread'i tanımlar.