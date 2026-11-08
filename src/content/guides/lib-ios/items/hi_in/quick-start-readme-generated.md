कमेंट विजेट दिखाने के लिए न्यूनतम सेटअप:

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

`"demo"` को अपनी FastComments tenant ID से बदलें। `urlId` उस पेज या थ्रेड की पहचान करता है जहाँ कमेंट्स स्टोर किए जाते हैं।