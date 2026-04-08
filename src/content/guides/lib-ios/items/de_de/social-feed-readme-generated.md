Das Feed-System ist ein separates SDK (`FastCommentsFeedSDK`) mit einer eigenen Ansicht.

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
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Die Feed-Ansicht enthält automatisch Pull-to-Refresh und unendliches Scrollen. Verwende `loadIfNeeded()` beim Wiedereintritt in den Bildschirm-Lifecycle, damit ein vorhandener oder wiederhergestellter Feed nicht zurück auf Seite 1 gesetzt wird.

### Beiträge erstellen

Verwende `FeedPostCreateView`, um ein Formular zum Erstellen eines Beitrags anzuzeigen:

```swift
@State private var showCreatePost = false

// In deinem View-Body:
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

### Auf Beiträge reagieren

Das SDK verwaltet Reaktionen mit optimistischen Updates:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Kommentare zu einem Beitrag öffnen

Verwende `CommentsSheet`, um Kommentare zu einem Feed-Beitrag anzuzeigen. Es erstellt intern eine `FastCommentsSDK`-Instanz unter Verwendung der Konfiguration des Feed-SDKs:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Benutzerklick behandeln
    })
}
```

Hinweis: `FeedPost` muss `Identifiable` entsprechen für `.sheet(item:)`. Füge diese Extension hinzu:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Tag-basierte Feed-Filterung

Implementiere das `TagSupplier`-Protokoll, um Feed-Beiträge nach Tags zu filtern:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Gib `nil` für einen ungefilterten globalen Feed zurück.

### Speichern und Wiederherstellen des Feed-Zustands

Bewahre den Paginierungszustand über View-Lifecycle-Ereignisse hinweg auf:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Wenn dein Bildschirm vorübergehend verschwindet, pausiert die Feed-Ansicht automatisch Live-Updates und setzt sie beim Wiedererscheinen fort, ohne geladene Beiträge zu löschen. Rufe `sdk.cleanup()` nur auf, wenn du die SDK-Instanz wirklich nicht mehr benötigst.

### Beiträge löschen

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---