Системата за емисия е отделен SDK (`FastCommentsFeedSDK`) с собствен изглед.

### Зареждане и показване на емисията

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
                // Покажете лист за споделяне
            }
            .onUserClick { context, userInfo, source in
                // Навигирайте до профила на потребителя
            }
            .onMediaClick { mediaItem, index in
                // Покажете изображение на цял екран
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Изгледът на емисията включва автоматично опресняване чрез издърпване и безкрайно превъртане.
Използвайте `loadIfNeeded()` при връщане в жизнения цикъл на екрана, за да предотвратите нулиране на вече заредена или възстановена емисия обратно на страница 1.

### Създаване на публикации

Използвайте `FeedPostCreateView`, за да покажете формуляр за създаване на публикация:

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

### Реакции към публикации

SDK обработва реакциите с оптимистични ъпдейти:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Проверете състоянието на реакцията
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Отваряне на коментари за публикация

Използвайте `CommentsSheet`, за да покажете коментарите за публикация в емисията. Той създава вътрешно инстанция `FastCommentsSDK`, използвайки конфигурацията на feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Обработете клик върху потребител
    })
}
```

Забележка: `FeedPost` трябва да съответства на `Identifiable` за `.sheet(item:)`. Добавете това разширение:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Филтриране на емисия по тагове

Реализирайте протокола `TagSupplier`, за да филтрирате публикациите в емисията по тагове:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Върнете `nil` за нефилтрирана глобална емисия.

### Запазване и възстановяване на състоянието на емисията

Запазете състоянието на пагинацията през събития от жизнения цикъл на изгледа:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Ако вашият екран изчезне временно, изгледът на емисията автоматично паузира живите ъпдейти и ги възобновява при повторно появяване без да изчиства заредените публикации. Извиквайте `sdk.cleanup()` само когато наистина сте приключили с инстанцията на SDK.

### Изтриване на публикации

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```