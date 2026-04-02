Das Feed-System ist ein separates SDK (`FastCommentsFeedSDK`) mit eigener Ansicht.

### Laden und Anzeigen des Feeds

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
                // Teilen-Dialog anzeigen
            }
            .onUserClick { context, userInfo, source in
                // Zum Benutzerprofil navigieren
            }
            .onMediaClick { mediaItem, index in
                // Vollbild-Bildbetrachter anzeigen
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Die Feed-Ansicht umfasst automatisch Pull-to-Refresh und unbegrenztes Scrollen.

### Erstellen von Beiträgen

Verwenden Sie `FeedPostCreateView`, um ein Formular zum Erstellen von Beiträgen anzuzeigen:

```swift
@State private var showCreatePost = false

// In your view body:
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

### Reagieren auf Beiträge

Das SDK verwaltet Reaktionen mit optimistischen Updates:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Kommentare zu einem Beitrag öffnen

Verwenden Sie `CommentsSheet`, um Kommentare für einen Feed-Beitrag anzuzeigen. Es erstellt intern eine `FastCommentsSDK`-Instanz unter Verwendung der Konfiguration des Feed-SDKs:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Benutzerklick behandeln
    })
}
```

Hinweis: `FeedPost` muss `Identifiable` für `.sheet(item:)` entsprechen. Fügen Sie diese Erweiterung hinzu:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Tag-basiertes Filtern des Feeds

Implementieren Sie das `TagSupplier`-Protokoll, um Feed-Beiträge nach Tags zu filtern:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Geben Sie `nil` für einen ungefilterten globalen Feed zurück.

### Speichern und Wiederherstellen des Feed-Zustands

Bewahren Sie den Paginierungszustand über den Lebenszyklus der Ansicht hinweg auf:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Beiträge löschen

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```