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
                // Predstavi delitveni list
            }
            .onUserClick { context, userInfo, source in
                // Odpri profil uporabnika
            }
            .onMediaClick { mediaItem, index in
                // Prikaži slikovni pregledovalnik na celotnem zaslonu
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Pogled feeda samodejno vključuje potegni-za-osvežitev in neskončno pomikanje.
Uporabite `loadIfNeeded()` ob ponovnem vstopu v življenjski cikel zaslona, da obstoječ ali obnovljen feed ne bo ponastavljen nazaj na stran 1.

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

### Odprtje komentarjev pri objavi

Uporabite `CommentsSheet` za prikaz komentarjev pri feed objavi. Notranje ustvari primerek `FastCommentsSDK` z uporabo konfiguracije feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Obravnava klika uporabnika
    })
}
```

Opomba: `FeedPost` se mora ujemati z `Identifiable` za `.sheet(item:)`. Dodajte to razširitev:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtriranje feeda po oznakah

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

Za nefiltriran globalni feed vrnite `nil`.

### Shranjevanje in obnavljanje stanja feeda

Ohranjajte stanje paginacije med dogodki življenjskega cikla pogleda:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Če vaš zaslon začasno izgine, pogled feeda samodejno začasno ustavi posodobitve v živo in jih nadaljuje ob ponovnem prikazu brez brisanja naloženih objav. Pokličite `sdk.cleanup()` le, ko ste res končali s primestkom SDK.

### Brisanje objav

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```