### मूलभूत उपयोग

```swift
struct CommentsPage: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "article-42",
            url: "https://example.com/article/42",
            pageTitle: "Article Title"
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

### वोट स्टाइल

डिफ़ॉल्ट वोट स्टाइल ऊपर/नीचे तीर दिखाता है। हार्ट-स्टाइल वोट के लिए `._1` पास करें:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| स्टाइल | रूप |
|-------|------------|
| `._0` | शुद्ध गिनती के साथ ऊपर/नीचे तीर बटन |
| `._1` | गिनती के साथ एकल हार्ट बटन |

### इवेंट कॉलबैक

उपयोगकर्ता इंटरैक्शन को संभालने के लिए मॉडिफ़ायर-स्टाइल कॉलबैक का उपयोग करें:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source .name या .avatar होता है
        print("Tapped \(userInfo.displayName)")
    }
```

### थीम लागू करना

SwiftUI एनवायरनमेंट के माध्यम से थीम पास करें:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

या इसे सीधे SDK पर सेट करें:

```swift
sdk.theme = FastCommentsTheme.modern
```

### सॉर्ट दिशा

```swift
sdk.defaultSortDirection = .nf  // नवीनतम पहले (डिफ़ॉल्ट)
sdk.defaultSortDirection = .of  // सबसे पुराना पहले
sdk.defaultSortDirection = .mr  // सर्वाधिक प्रासंगिक
```