Sustav feeda je zaseban SDK (`FastCommentsFeedSDK`) s vlastitim prikazom.

### Učitavanje i prikaz feeda

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
                // Prikaži dijaloški okvir za dijeljenje
            }
            .onUserClick { context, userInfo, source in
                // Navigiraj do korisničkog profila
            }
            .onMediaClick { mediaItem, index in
                // Prikaži pregled slike preko cijelog zaslona
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Prikaz feeda automatski uključuje povlačenje za osvježavanje i beskonačno skrolanje.

### Kreiranje objava

Koristite `FeedPostCreateView` za prikaz obrasca za stvaranje objave:

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

SDK upravlja reakcijama koristeći optimistička ažuriranja:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Provjeri stanje reakcije
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Otvaranje komentara na objavu

Koristite `CommentsSheet` za prikaz komentara za objavu u feedu. Interno stvara instancu `FastCommentsSDK` koristeći konfiguraciju feed SDK-a:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Obradi klik korisnika
    })
}
```

Napomena: `FeedPost` mora implementirati `Identifiable` radi korištenja `.sheet(item:)`. Dodajte ovo proširenje:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtriranje feeda prema tagovima

Implementirajte protokol `TagSupplier` za filtriranje objava u feedu prema tagovima:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Vrati `nil` za nefiltrirani globalni feed.

### Spremanje i vraćanje stanja feeda

Sačuvajte stanje paginacije kroz događaje životnog ciklusa prikaza:

```swift
let state = sdk.savePaginationState()
// Kasnije...
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