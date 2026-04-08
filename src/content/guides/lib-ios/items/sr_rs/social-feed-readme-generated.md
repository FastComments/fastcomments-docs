Sistem feed-a je odvojen SDK (`FastCommentsFeedSDK`) sa sopstvenim prikazom.

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
                // Navigirajte do profila korisnika
            }
            .onMediaClick { mediaItem, index in
                // Prikaži pregled slika preko celog ekrana
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Prikaz feed-a automatski uključuje povlačenje za osvežavanje i beskonačno skrolovanje.
Koristite `loadIfNeeded()` pri ponovnom ulasku u životni ciklus ekrana kako postojeći ili vraćeni feed ne bi bio resetovan na stranicu 1.

### Kreiranje objava

Koristite `FeedPostCreateView` da prikažete formu za kreiranje objave:

```swift
@State private var showCreatePost = false

// U telu vašeg prikaza:
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

SDK obrađuje reakcije sa optimističkim ažuriranjima:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Proverite stanje reakcije
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Otvaranje komentara na objavi

Koristite `CommentsSheet` za prikaz komentara za feed objavu. Interno kreira instancu `FastCommentsSDK` koristeći konfiguraciju feed SDK-a:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Obradite klik korisnika
    })
}
```

Napomena: `FeedPost` mora da implementira `Identifiable` za `.sheet(item:)`. Dodajte ovo proširenje:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtriranje feed-a po tagovima

Implementirajte protokol `TagSupplier` da filtrirate feed objave po tagovima:

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

Sačuvajte stanje paginacije kroz događaje životnog ciklusa prikaza:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Ako se vaš ekran privremeno sakrije, prikaz feed-a automatski pauzira ažuriranja uživo i nastavlja ih prilikom ponovnog pojavljivanja bez brisanja učitanih objava. Pozovite `sdk.cleanup()` samo kada ste zaista završili sa instancom SDK-a.

### Brisanje objava

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---