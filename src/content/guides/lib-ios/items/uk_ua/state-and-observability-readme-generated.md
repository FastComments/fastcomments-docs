Обидва `FastCommentsSDK` та `FastCommentsFeedSDK` — це класи `ObservableObject` з властивостями `@Published`. Ви можете спостерігати за ними у ваших SwiftUI представленнях для реактивного оновлення інтерфейсу.

### Опубліковані властивості FastCommentsSDK

| Властивість | Тип | Опис |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Загальна кількість коментарів на сервері |
| `newRootCommentCount` | `Int` | Накопичені нові коментарі (коли `showLiveRightAway` встановлено в false) |
| `currentUser` | `UserSessionInfo?` | Поточний автентифікований користувач |
| `isSiteAdmin` | `Bool` | Чи є поточний користувач адміністратором сайту |
| `isClosed` | `Bool` | Чи закрито тред коментарів |
| `hasBillingIssue` | `Bool` | Чи існує проблема з білінгом |
| `isLoading` | `Bool` | Чи виконується мережевий запит |
| `hasMore` | `Bool` | Чи існують додаткові сторінки коментарів |
| `blockingErrorMessage` | `String?` | Помилка, яка перешкоджає роботі інтерфейсу |
| `warningMessage` | `String?` | Попереджувальне повідомлення, що не блокує |
| `isDemo` | `Bool` | Чи працює у демонстраційному режимі |
| `commentsVisible` | `Bool` | Перемикач видимості коментарів |
| `toolbarEnabled` | `Bool` | Чи відображається панель форматування |

### Опубліковані властивості FastCommentsFeedSDK

| Властивість | Тип | Опис |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Наразі завантажені пости стрічки |
| `hasMore` | `Bool` | Чи існують додаткові сторінки |
| `currentUser` | `UserSessionInfo?` | Поточний автентифікований користувач |
| `blockingErrorMessage` | `String?` | Повідомлення про помилку, що блокує |
| `isLoading` | `Bool` | Чи виконується мережевий запит |
| `newPostsCount` | `Int` | Кількість нових дописів з моменту останнього завантаження |

### Дерево коментарів

Дерево коментарів доступне через `sdk.commentsTree`:

```swift
// Плоский список видимих вузлів для відображення
sdk.commentsTree.visibleNodes

// Пошук коментаря за ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---