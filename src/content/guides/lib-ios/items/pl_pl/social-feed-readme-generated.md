System kanału jest oddzielnym SDK (`FastCommentsFeedSDK`) z własnym widokiem.

### Ładowanie i wyświetlanie kanału

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
                // Pokaż przeglądarkę obrazów w trybie pełnoekranowym
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Widok kanału automatycznie obsługuje pull-to-refresh i nieskończone przewijanie.
Użyj `loadIfNeeded()` przy ponownym wejściu w cykl życia ekranu, aby istniejący lub przywrócony kanał nie został zresetowany do strony 1.

### Tworzenie postów

Użyj `FeedPostCreateView`, aby wyświetlić formularz tworzenia postu:

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

### Otwieranie komentarzy dla postu

Użyj `CommentsSheet`, aby wyświetlić komentarze dla posta z kanału. Tworzy on instancję `FastCommentsSDK` wewnętrznie, używając konfiguracji feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Obsłuż kliknięcie użytkownika
    })
}
```

Uwaga: `FeedPost` musi implementować `Identifiable` dla `.sheet(item:)`. Dodaj to rozszerzenie:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtrowanie kanału na podstawie tagów

Zaimplementuj protokół `TagSupplier`, aby filtrować posty kanału według tagów:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Zwróć `nil` dla niefiltrowanego globalnego kanału.

### Zapis i przywracanie stanu kanału

Zachowaj stan paginacji pomiędzy zdarzeniami cyklu życia widoku:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Jeśli twój ekran znika tymczasowo, widok kanału automatycznie wstrzymuje aktualizacje na żywo i wznawia je po ponownym pojawieniu się bez usuwania załadowanych postów. Wywołuj `sdk.cleanup()` tylko wtedy, gdy naprawdę kończysz korzystanie z instancji SDK.

### Usuwanie postów

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```