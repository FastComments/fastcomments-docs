Oba `FastCommentsSDK` i `FastCommentsFeedSDK` su klase `ObservableObject` sa `@Published` svojstvima. Možete ih posmatrati u svojim SwiftUI prikazima za reaktivna ažuriranja UI-ja.

### Objavljena svojstva FastCommentsSDK

| Svojstvo | Tip | Opis |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Ukupan broj komentara na serveru |
| `newRootCommentCount` | `Int` | Privremeno pohranjeni novi komentari (kada je `showLiveRightAway` false) |
| `currentUser` | `UserSessionInfo?` | Trenutni autentifikovani korisnik |
| `isSiteAdmin` | `Bool` | Da li je trenutni korisnik administrator sajta |
| `isClosed` | `Bool` | Da li je nit komentara zatvorena |
| `hasBillingIssue` | `Bool` | Da li postoji problem sa naplatom |
| `isLoading` | `Bool` | Da li je u toku mrežni zahtev |
| `hasMore` | `Bool` | Da li postoje dodatne stranice komentara |
| `blockingErrorMessage` | `String?` | Greška koja sprečava funkcionisanje UI-ja |
| `warningMessage` | `String?` | Upozoravajuća poruka koja ne blokira |
| `isDemo` | `Bool` | Da li radi u demo režimu |
| `commentsVisible` | `Bool` | Prekidač vidljivosti komentara |
| `toolbarEnabled` | `Bool` | Da li je alatna traka za formatiranje prikazana |

### Objavljena svojstva FastCommentsFeedSDK

| Svojstvo | Tip | Opis |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Trenutno učitani postovi feeda |
| `hasMore` | `Bool` | Da li postoje dodatne stranice |
| `currentUser` | `UserSessionInfo?` | Trenutni autentifikovani korisnik |
| `blockingErrorMessage` | `String?` | Blokirajuća poruka o grešci |
| `isLoading` | `Bool` | Da li je u toku mrežni zahtev |
| `newPostsCount` | `Int` | Broj novih postova od posljednjeg učitavanja |

### Comment Tree

Stablo komentara je dostupno putem `sdk.commentsTree`:

```swift
// Ravna lista vidljivih čvorova za renderovanje
sdk.commentsTree.visibleNodes

// Pronađi komentar po ID-u
sdk.commentsTree.commentsById["comment-id"]
```