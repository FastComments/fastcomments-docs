Tako `FastCommentsSDK` kot `FastCommentsFeedSDK` sta razreda `ObservableObject` z lastnostmi `@Published`. V svojih SwiftUI pogledih jih lahko opazujete za reaktivne posodobitve uporabniškega vmesnika.

### Objavljene lastnosti FastCommentsSDK

| Lastnost | Tip | Opis |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Skupno število komentarjev na strežniku |
| `newRootCommentCount` | `Int` | Vmesno shranjeni novi komentarji (ko je `showLiveRightAway` false) |
| `currentUser` | `UserSessionInfo?` | Trenutno avtenticiran uporabnik |
| `isSiteAdmin` | `Bool` | Ali je trenutni uporabnik skrbnik strani |
| `isClosed` | `Bool` | Ali je nit komentarjev zaprta |
| `hasBillingIssue` | `Bool` | Ali obstaja težava z obračunom |
| `isLoading` | `Bool` | Ali poteka omrežni zahtevek |
| `hasMore` | `Bool` | Ali obstajajo dodatne strani komentarjev |
| `blockingErrorMessage` | `String?` | Napaka, ki preprečuje delovanje uporabniškega vmesnika |
| `warningMessage` | `String?` | Neblokirajoče opozorilno sporočilo |
| `isDemo` | `Bool` | Ali se izvaja v demo načinu |
| `commentsVisible` | `Bool` | Preklop za vidnost komentarjev |
| `toolbarEnabled` | `Bool` | Ali je prikazana orodna vrstica za oblikovanje |

### Objavljene lastnosti FastCommentsFeedSDK

| Lastnost | Tip | Opis |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Trenutno naloženi prispevki v feedu |
| `hasMore` | `Bool` | Ali obstajajo dodatne strani |
| `currentUser` | `UserSessionInfo?` | Trenutno avtenticiran uporabnik |
| `blockingErrorMessage` | `String?` | Blokirajoče sporočilo o napaki |
| `isLoading` | `Bool` | Ali poteka omrežni zahtevek |
| `newPostsCount` | `Int` | Število novih prispevkov od zadnjega nalaganja |

### Drevo komentarjev

Drevo komentarjev je dostopno preko `sdk.commentsTree`:

```swift
// Seznam vidnih vozlišč za prikaz
sdk.commentsTree.visibleNodes

// Poiščite komentar po ID-ju
sdk.commentsTree.commentsById["comment-id"]
```

---
---