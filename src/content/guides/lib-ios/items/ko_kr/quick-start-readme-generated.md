댓글 위젯을 표시하기 위한 최소 설정:

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

`"demo"`를 여러분의 FastComments 테넌트 ID로 교체하세요. `urlId`는 댓글이 저장되는 페이지 또는 스레드를 식별합니다.

---
---