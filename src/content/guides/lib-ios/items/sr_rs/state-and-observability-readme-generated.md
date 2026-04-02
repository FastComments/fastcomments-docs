Oba `FastCommentsSDK` и `FastCommentsFeedSDK` су класе `ObservableObject` са `@Published` својствима. Можете их посматрати у својим SwiftUI приказима за реактивна ажурирања корисничког интерфејса.

### Objavljena svojstva FastCommentsSDK

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Ukupan broj komentara na serveru |
| `newRootCommentCount` | `Int` | Privremeno sačuvani novi komentari (kada је `showLiveRightAway` false) |
| `currentUser` | `UserSessionInfo?` | Trenutno autentifikovani korisnik |
| `isSiteAdmin` | `Bool` | Da li је trenutni korisnik administrator sajta |
| `isClosed` | `Bool` | Da li је nit komentara zatvorena |
| `hasBillingIssue` | `Bool` | Da li постоји problem са наплатом |
| `isLoading` | `Bool` | Да ли је мрежни захтев у току |
| `hasMore` | `Bool` | Да ли постоје још странице са коментарима |
| `blockingErrorMessage` | `String?` | Грешка која спречава функционисање корисничког интерфејса |
| `warningMessage` | `String?` | Неблокирајућа порука упозорења |
| `isDemo` | `Bool` | Да ли се покреће у демо режиму |
| `commentsVisible` | `Bool` | Прекидач за видљивост коментара |
| `toolbarEnabled` | `Bool` | Да ли је алатна трака за форматирање приказана |

### Objavljena svojstva FastCommentsFeedSDK

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Тренутно учитане објаве фида |
| `hasMore` | `Bool` | Да ли постоје још странице |
| `currentUser` | `UserSessionInfo?` | Тренутно аутентификовани корисник |
| `blockingErrorMessage` | `String?` | Блокирајућа порука о грешци |
| `isLoading` | `Bool` | Да ли је мрежни захтев у току |
| `newPostsCount` | `Int` | Број нових објава од последњег учитавања |

### Stablo komentara

Stablo komentara je dostupno preko `sdk.commentsTree`:

```swift
// Ravna lista vidljivih čvorova za renderovanje
sdk.commentsTree.visibleNodes

// Pronađi komentar po ID-u
sdk.commentsTree.commentsById["comment-id"]
```