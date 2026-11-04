Sistem feed-a je zaseban SDK (`FastCommentsFeedSDK`) sa sopstvenim prikazom.

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
                // Prikaži dijalog za deljenje
            }
            .onUserClick { context, userInfo, source in
                // Otvori korisnički profil
            }
            .onMediaClick { mediaItem, index in
                // Prikaži prikaz slike preko celog ekrana
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Prikaz feeda automatski podržava povlačenje za osvežavanje i beskonačno skrolovanje.
Koristite `loadIfNeeded()` pri ponovnom učitavanju ekrana da postojeći ili vraćeni feed ne bi bio resetovan na stranicu 1.

### Kreiranje objava

Koristite `FeedPostCreateView` za prikaz forme za kreiranje objave:

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

### Reagovanje na objave

SDK upravlja reakcijama koristeći optimističke promene:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Proveri stanje reakcije
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Otvaranje komentara na objavu

Koristite `CommentsSheet` za prikaz komentara za objavu u feedu. Pri tome interno kreira `FastCommentsSDK` instancu koristeći konfiguraciju feed SDK-a:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Obradi klik na korisnika
    })
}
```

Napomena: `FeedPost` mora da se pridržava `Identifiable` za `.sheet(item:)`. Dodajte ovo proširenje:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtriranje feeda po tagovima

Implementirajte `TagSupplier` protokol da filtrirate objave u feedu po tagovima:

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

### Čuvanje i vraćanje stanja feeda

Sačuvajte stanje paginacije kroz događaje životnog ciklusa prikaza:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Ako se vaš ekran privremeno sakrije, prikaz feeda automatski pauzira live ažuriranja i nastavlja ih kada se ponovo pojavi, bez brisanja učitanih objava. Pozovite `sdk.cleanup()` samo kada ste zaista završili sa tom SDK instancom.

### Brisanje objava

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---