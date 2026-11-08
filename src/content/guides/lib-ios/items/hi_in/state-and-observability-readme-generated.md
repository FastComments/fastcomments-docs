`FastCommentsSDK` और `FastCommentsFeedSDK` दोनों `@Published` प्रॉपर्टीज़ वाली `ObservableObject` क्लासेस हैं। आप अपने SwiftUI व्यूज़ में इन्हें रिएक्टिव UI अपडेट्स के लिए observe कर सकते हैं।

### FastCommentsSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | सर्वर पर कुल कमेंट संख्या |
| `newRootCommentCount` | `Int` | बफ़र की गई नई कमेंट्स (जब `showLiveRightAway` false हो) |
| `currentUser` | `UserSessionInfo?` | वर्तमान प्रमाणित यूज़र |
| `isSiteAdmin` | `Bool` | क्या वर्तमान यूज़र साइट एडमिन है |
| `isClosed` | `Bool` | क्या कमेंट थ्रेड बंद है |
| `hasBillingIssue` | `Bool` | क्या बिलिंग में कोई समस्या है |
| `isLoading` | `Bool` | क्या कोई नेटवर्क रिक्वेस्ट चल रही है |
| `hasMore` | `Bool` | क्या कमेंट्स के और पेज मौजूद हैं |
| `blockingErrorMessage` | `String?` | वह एरर जो UI के काम करने से रोकता है |
| `warningMessage` | `String?` | नॉन-ब्लॉकिंग चेतावनी संदेश |
| `isDemo` | `Bool` | क्या डेमो मोड में चल रहा है |
| `commentsVisible` | `Bool` | कमेंट विज़िबिलिटी के लिए टॉगल |
| `toolbarEnabled` | `Bool` | क्या फ़ॉर्मैटिंग टूलबार दिखाया जाता है |

### FastCommentsFeedSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | वर्तमान में लोड की गई फ़ीड पोस्ट्स |
| `hasMore` | `Bool` | क्या और पेज मौजूद हैं |
| `currentUser` | `UserSessionInfo?` | वर्तमान प्रमाणित यूज़र |
| `blockingErrorMessage` | `String?` | ब्लॉकिंग एरर संदेश |
| `isLoading` | `Bool` | क्या कोई नेटवर्क रिक्वेस्ट चल रही है |
| `newPostsCount` | `Int` | पिछली लोड के बाद नई पोस्ट्स की संख्या |

### Comment Tree

कमेंट ट्री को `sdk.commentsTree` के ज़रिए एक्सेस किया जा सकता है:

```swift
// Flat list of visible nodes for rendering
sdk.commentsTree.visibleNodes

// Lookup a comment by ID
sdk.commentsTree.commentsById["comment-id"]
```

---