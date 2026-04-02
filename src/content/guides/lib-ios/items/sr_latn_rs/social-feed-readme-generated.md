Sistem feed-a je zaseban SDK (`FastCommentsFeedSDK`) sa sopstvenim prikazom.

### Učitavanje i prikaz feed-a

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
                // Prikaži sheet za deljenje
            }
            .onUserClick { context, userInfo, source in
                // Navigiraj do korisničkog profila
            }
            .onMediaClick { mediaItem, index in
                // Prikaži pregledač slike preko celog ekrana
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Prikaz feed-a uključuje pull-to-refresh i infinite scroll automatski.

### Kreiranje objava

Koristite `FeedPostCreateView` za prikaz formulara za kreiranje objave:

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

### Reakcije na objave

SDK upravlja reakcijama sa optimističkim ažuriranjima:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Otvaranje komentara na objavi

Koristite `CommentsSheet` za prikaz komentara za feed objavu. On kreira `FastCommentsSDK` instancu interno koristeći konfiguraciju feed SDK-a:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Obradi klik korisnika
    })
}
```

Napomena: `FeedPost` mora da implementira `Identifiable` za `.sheet(item:)`. Dodajte ovo proširenje:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtriranje feed-a po tagovima

Implementirajte `TagSupplier` protokol da filtrirate feed objave po tagovima:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Vratite `nil` za nefiltriran globalni feed.

### Čuvanje i vraćanje stanja feed-a

Sačuvajte stanje paginacije preko događaja životnog ciklusa prikaza:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Brisanje objava

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---