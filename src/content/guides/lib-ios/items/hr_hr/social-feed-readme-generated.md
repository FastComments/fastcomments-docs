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
                // Prikaži prozor za dijeljenje
            }
            .onUserClick { context, userInfo, source in
                // Navigiraj na korisnički profil
            }
            .onMediaClick { mediaItem, index in
                // Prikaži preglednik slika preko cijelog zaslona
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Prikaz feeda automatski uključuje povlačenje za osvježavanje i beskonačno pomicanje. Koristite `loadIfNeeded()` pri ponovnom ulasku u životni ciklus zaslona kako biste spriječili da postojeći ili vraćeni feed bude resetiran na stranicu 1.

### Kreiranje objava

Upotrijebite `FeedPostCreateView` za prikaz obrasca za kreiranje objave:

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

SDK upravlja reakcijama s optimističkim ažuriranjima:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Otvaranje komentara za objavu

Upotrijebite `CommentsSheet` za prikaz komentara za objavu u feedu. Interno kreira instancu `FastCommentsSDK` koristeći konfiguraciju feed SDK-a:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Obradi klik korisnika
    })
}
```

Napomena: `FeedPost` mora zadovoljiti `Identifiable` za `.sheet(item:)`. Dodajte ovo proširenje:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtriranje feeda po oznakama

Implementirajte protokol `TagSupplier` za filtriranje objava u feedu po oznakama:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Vratite `nil` za nefiltrirani globalni feed.

### Spremanje i vraćanje stanja feeda

Sačuvajte stanje paginacije kroz događaje životnog ciklusa prikaza:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Ako se vaš zaslon privremeno sakrije, prikaz feeda automatski pauzira uživo ažuriranja i nastavlja ih pri ponovnom pojavljivanju bez brisanja učitanih objava. Pozovite `sdk.cleanup()` samo kada ste zaista gotovi s instancom SDK-a.

### Brisanje objava

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```