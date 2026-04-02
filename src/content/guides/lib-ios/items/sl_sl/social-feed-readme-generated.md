Sistem feeda je ločen SDK (`FastCommentsFeedSDK`) s svojim pogledom.

### Nalaganje in prikaz feeda

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
                // Prikaži delilni list
            }
            .onUserClick { context, userInfo, source in
                // Pojdi na profil uporabnika
            }
            .onMediaClick { mediaItem, index in
                // Prikaži pregledovalnik slike na celotnem zaslonu
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Pogled feeda samodejno podpira vlečenje za osvežitev in neskončno pomikanje.

### Ustvarjanje objav

Uporabite `FeedPostCreateView` za prikaz obrazca za ustvarjanje objave:

```swift
@State private var showCreatePost = false

// V telesu vašega pogleda:
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

SDK obravnava reakcije z optimističnimi posodobitvami:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Preveri stanje reakcije
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Odpiranje komentarjev na objavi

Uporabite `CommentsSheet` za prikaz komentarjev pri feed objavi. Znotraj ustvari primer `FastCommentsSDK` z uporabo konfiguracije feed SDK-ja:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Obdelaj klik uporabnika
    })
}
```

Opomba: `FeedPost` mora ustrezati protokolu `Identifiable` za `.sheet(item:)`. Dodajte to razširitev:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtriranje feeda na podlagi oznak

Implementirajte protokol `TagSupplier` za filtriranje feed objav po oznakah:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Vrnite `nil` za nefiltriran globalni feed.

### Shranjevanje in obnavljanje stanja feeda

Ohranite stanje paginacije med dogodki življenjskega cikla pogleda:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Brisanje objav

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```