И двете `FastCommentsSDK` и `FastCommentsFeedSDK` са класове `ObservableObject` с свойства `@Published`. Можете да наблюдавате тези свойства във вашите SwiftUI изгледи за реактивни актуализации на потребителския интерфейс.

### Публикувани свойства на FastCommentsSDK

| Свойство | Тип | Описание |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Общ брой коментари на сървъра |
| `newRootCommentCount` | `Int` | Буферирани нови коментари (когато `showLiveRightAway` е false) |
| `currentUser` | `UserSessionInfo?` | Текущ удостоверен потребител |
| `isSiteAdmin` | `Bool` | Дали текущият потребител е администратор на сайта |
| `isClosed` | `Bool` | Дали нишката с коментари е затворена |
| `hasBillingIssue` | `Bool` | Има ли проблем с фактурирането |
| `isLoading` | `Bool` | Дали се изпълнява мрежова заявка |
| `hasMore` | `Bool` | Има ли още страници с коментари |
| `blockingErrorMessage` | `String?` | Грешка, която предотвратява функционирането на потребителския интерфейс |
| `warningMessage` | `String?` | Неблокиращо предупредително съобщение |
| `isDemo` | `Bool` | Дали работи в демонстрационен режим |
| `commentsVisible` | `Bool` | Превключвател за видимостта на коментарите |
| `toolbarEnabled` | `Bool` | Дали панелът за форматиране е показан |

### Публикувани свойства на FastCommentsFeedSDK

| Свойство | Тип | Описание |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | В момента заредени публикации във фийда |
| `hasMore` | `Bool` | Има ли още страници |
| `currentUser` | `UserSessionInfo?` | Текущ удостоверен потребител |
| `blockingErrorMessage` | `String?` | Блокиращо съобщение за грешка |
| `isLoading` | `Bool` | Дали се изпълнява мрежова заявка |
| `newPostsCount` | `Int` | Брой нови публикации от последното зареждане |

### Дървото на коментарите

Дървото на коментарите е достъпно чрез `sdk.commentsTree`:

```swift
// Плосък списък с видимите възли за рендиране
sdk.commentsTree.visibleNodes

// Намиране на коментар по ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---