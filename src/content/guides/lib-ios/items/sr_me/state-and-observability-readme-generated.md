Обa `FastCommentsSDK` и `FastCommentsFeedSDK` су класе `ObservableObject` са `@Published` својствима. Можете посматрати ове у вашим SwiftUI приказима за реактивна ажурирања интерфејса.

### FastCommentsSDK Објављена својства

| Својство | Тип | Опис |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Укупни број коментара на серверу |
| `newRootCommentCount` | `Int` | Буферирани нови коментари (када је `showLiveRightAway` false) |
| `currentUser` | `UserSessionInfo?` | Тренутни аутентификовани корисник |
| `isSiteAdmin` | `Bool` | Да ли је тренутни корисник админ сајта |
| `isClosed` | `Bool` | Да ли је нит коментара затворена |
| `hasBillingIssue` | `Bool` | Да ли постоји проблем са плаћањем |
| `isLoading` | `Bool` | Да ли је мрежни захтев у току |
| `hasMore` | `Bool` | Да ли постоје додатне странице коментара |
| `blockingErrorMessage` | `String?` | Грешка која спречава рад корисничког интерфејса |
| `warningMessage` | `String?` | Упозоравајућа порука која не блокира рад |
| `isDemo` | `Bool` | Да ли се покреће у демо режиму |
| `commentsVisible` | `Bool` | Прекидач видљивости коментара |
| `toolbarEnabled` | `Bool` | Да ли је трака за форматирање приказана |

### FastCommentsFeedSDK Објављена својства

| Својство | Тип | Опис |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Тренутно учитани постови фида |
| `hasMore` | `Bool` | Да ли постоје додатне странице |
| `currentUser` | `UserSessionInfo?` | Тренутни аутентификовани корисник |
| `blockingErrorMessage` | `String?` | Блокирајућа порука о грешци |
| `isLoading` | `Bool` | Да ли је мрежни захтев у току |
| `newPostsCount` | `Int` | Број нових постова од последњег учитавања |

### Дрво коментара

Дрво коментара је доступно преко `sdk.commentsTree`:

```swift
// Плосна листа видљивих чворова за рендеровање
sdk.commentsTree.visibleNodes

// Претрага коментара по ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---