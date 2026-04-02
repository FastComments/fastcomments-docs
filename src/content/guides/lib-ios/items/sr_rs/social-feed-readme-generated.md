Систем фида је посебан SDK (`FastCommentsFeedSDK`) са својим приказом.

### Учитавање и приказивање фида

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
                // Прикажи дијалог за дељење
            }
            .onUserClick { context, userInfo, source in
                // Навигирај до корисничког профила
            }
            .onMediaClick { mediaItem, index in
                // Прикажи прегледач слика преко целог екрана
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Преглед фида аутоматски садржи повлачење за освежавање и бесконачно скроловање.

### Креирање објава

Користите `FeedPostCreateView` да прикажете форму за креирање објаве:

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

### Реакције на објаве

SDK рукује реакцијама са оптимистичким ажурирањима:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Отварање коментара за објаву

Користите `CommentsSheet` за приказ коментара за објаву из фида. Он интерно креира инстанцу `FastCommentsSDK` користећи конфигурацију фид SDK-а:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Обради кликање на корисника
    })
}
```

Напомена: `FeedPost` мора да подржава `Identifiable` за `.sheet(item:)`. Додајте ово проширење:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Филтрирање фида по таговима

Имплементирајте протокол `TagSupplier` да бисте филтрирали објаве у фиду по таговима:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Вратите `nil` да бисте добили нефилтрирани глобални фид.

### Чување и враћање стања фида

Чувајте стање пагинације кроз догађаје животног циклуса приказа:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Брисање објава

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```