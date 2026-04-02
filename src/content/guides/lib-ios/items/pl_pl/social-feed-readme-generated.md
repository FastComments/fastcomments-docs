System feedów to oddzielne SDK (`FastCommentsFeedSDK`) z własnym widokiem.

### Ładowanie i wyświetlanie feedu

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
                // Pokaż arkusz udostępniania
            }
            .onUserClick { context, userInfo, source in
                // Przejdź do profilu użytkownika
            }
            .onMediaClick { mediaItem, index in
                // Pokaż przeglądarkę obrazów na pełnym ekranie
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Widok feedu automatycznie obsługuje pull-to-refresh i nieskończone przewijanie.

### Tworzenie postów

Użyj `FeedPostCreateView`, aby pokazać formularz tworzenia postu:

```swift
@State private var showCreatePost = false

// W ciele widoku:
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

### Reakcje na posty

SDK obsługuje reakcje z optymistycznymi aktualizacjami:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Otwieranie komentarzy do posta

Użyj `CommentsSheet`, aby wyświetlić komentarze dla posta z feedu. Tworzy ona wewnętrznie instancję `FastCommentsSDK` używając konfiguracji feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Obsłuż kliknięcie użytkownika
    })
}
```

Uwaga: `FeedPost` musi implementować protokół `Identifiable` dla `.sheet(item:)`. Dodaj to rozszerzenie:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtrowanie feedu według tagów

Zaimplementuj protokół `TagSupplier`, aby filtrować posty feedu według tagów:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Zwróć `nil`, aby uzyskać niefiltrowany, globalny feed.

### Zapisywanie i przywracanie stanu feedu

Zachowaj stan paginacji pomiędzy zdarzeniami cyklu życia widoku:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Usuwanie postów

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---