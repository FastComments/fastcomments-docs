फ़ीड सिस्टम एक अलग SDK (`FastCommentsFeedSDK`) है जिसका अपना व्यू होता है।

### फ़ीड लोड करना और दिखाना

```swift
struct FeedPage: View {
    @StateObject private var sdk: FastCommentsFeedSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-feed",
            sso: ssoToken
        )
        return FastCommentsFeedSDK(config: config)
    }()

    @State private var commentsPost: FeedPost?

    var body: some View {
        FastCommentsFeedView(sdk: sdk)
            .onPostSelected { post in
                commentsPost = post
            }
            .onCommentsRequested { post in
                commentsPost = post
            }
            .onSharePost { post in
                // शेयर शीट प्रस्तुत करें
            }
            .onUserClick { context, userInfo, source in
                // यूज़र प्रोफ़ाइल पर नेविगेट करें
            }
            .onMediaClick { mediaItem, index in
                // फ़ुल-स्क्रीन इमेज व्यूअर प्रस्तुत करें
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

फ़ीड व्यू में पुल-टू-रिफ्रेश और इनफिनिट स्क्रॉल स्वचालित रूप से शामिल हैं।
स्क्रीन लाइफ़साइकिल में पुनः प्रवेश के लिए `loadIfNeeded()` का उपयोग करें ताकि कोई मौजूदा या पुनर्स्थापित फ़ीड पेज 1 पर रीसेट न हो जाए।

### पोस्ट बनाना

पोस्ट निर्माण फ़ॉर्म प्रस्तुत करने के लिए `FeedPostCreateView` का उपयोग करें:

```swift
@State private var showCreatePost = false

// अपने व्यू बॉडी में:
.sheet(isPresented: $showCreatePost) {
    FeedPostCreateView(
        sdk: sdk,
        onPostCreated: { post in
            showCreatePost = false
            Task { try? await sdk.refresh() }
        },
        onCancelled: {
            showCreatePost = false
        }
    )
}
```

### पोस्ट पर प्रतिक्रिया देना

SDK ऑप्टिमिस्टिक अपडेट के साथ प्रतिक्रियाओं को संभालता है:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// प्रतिक्रिया स्थिति जाँचें
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### किसी पोस्ट पर टिप्पणियाँ खोलना

फ़ीड पोस्ट के लिए टिप्पणियाँ दिखाने के लिए `CommentsSheet` का उपयोग करें। यह फ़ीड SDK के कॉन्फ़िग का उपयोग करके आंतरिक रूप से एक `FastCommentsSDK` इंस्टेंस बनाता है:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // यूज़र क्लिक को संभालें
    })
}
```

नोट: `.sheet(item:)` के लिए `FeedPost` को `Identifiable` के अनुरूप होना चाहिए। यह एक्सटेंशन जोड़ें:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### टैग-आधारित फ़ीड फ़िल्टरिंग

टैग द्वारा फ़ीड पोस्ट फ़िल्टर करने के लिए `TagSupplier` प्रोटोकॉल लागू करें:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

बिना फ़िल्टर वाली ग्लोबल फ़ीड के लिए `nil` लौटाएँ।

### फ़ीड स्थिति को सहेजना और पुनर्स्थापित करना

व्यू लाइफ़साइकिल इवेंट्स के दौरान पेजिनेशन स्थिति को सुरक्षित रखें:

```swift
let state = sdk.savePaginationState()
// बाद में...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

यदि आपकी स्क्रीन अस्थायी रूप से गायब हो जाती है, तो फ़ीड व्यू लाइव अपडेट को स्वचालित रूप से रोक देता है और लोड की गई पोस्ट साफ़ किए बिना पुनः दिखने पर उन्हें फिर से शुरू कर देता है। `sdk.cleanup()` को केवल तब कॉल करें जब आप SDK इंस्टेंस के साथ वास्तव में काम पूरा कर चुके हों।

### पोस्ट हटाना

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---