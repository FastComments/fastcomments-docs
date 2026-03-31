I `FastCommentsSDK` i `FastCommentsFeedSDK` su klase `ObservableObject` s `@Published` svojstvima. Možete ih promatrati u svojim SwiftUI prikazima za reaktivna ažuriranja korisničkog sučelja.

### FastCommentsSDK Objavljena svojstva

| Svojstvo | Tip | Opis |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Ukupan broj komentara na poslužitelju |
| `newRootCommentCount` | `Int` | Privremeno pohranjeni novi komentari (kad je `showLiveRightAway` false) |
| `currentUser` | `UserSessionInfo?` | Trenutno autentificirani korisnik |
| `isSiteAdmin` | `Bool` | Je li trenutni korisnik administrator stranice |
| `isClosed` | `Bool` | Je li nit komentara zatvorena |
| `hasBillingIssue` | `Bool` | Postoji li problem s naplatom |
| `isLoading` | `Bool` | Je li mrežni zahtjev u tijeku |
| `hasMore` | `Bool` | Postoje li dodatne stranice komentara |
| `blockingErrorMessage` | `String?` | Pogreška koja sprječava rad korisničkog sučelja |
| `warningMessage` | `String?` | Upozoravajuća poruka koja ne blokira |
| `isDemo` | `Bool` | Radi li se u demo načinu |
| `commentsVisible` | `Bool` | Prekidač za vidljivost komentara |
| `toolbarEnabled` | `Bool` | Je li alatna traka za formatiranje prikazana |

### FastCommentsFeedSDK Objavljena svojstva

| Svojstvo | Tip | Opis |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Trenutno učitani postovi u feedu |
| `hasMore` | `Bool` | Postoje li dodatne stranice |
| `currentUser` | `UserSessionInfo?` | Trenutno autentificirani korisnik |
| `blockingErrorMessage` | `String?` | Blokirajuća poruka o pogrešci |
| `isLoading` | `Bool` | Je li mrežni zahtjev u tijeku |
| `newPostsCount` | `Int` | Broj novih postova od posljednjeg učitavanja |

### Stablo komentara

Stablo komentara dostupno je putem `sdk.commentsTree`:

```swift
// Ravan popis vidljivih čvorova za renderiranje
sdk.commentsTree.visibleNodes

// Potraži komentar po ID-u
sdk.commentsTree.commentsById["comment-id"]
```

---
---