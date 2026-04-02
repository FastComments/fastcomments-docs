Oba `FastCommentsSDK` i `FastCommentsFeedSDK` su klase `ObservableObject` sa `@Published` svojstvima. Možete ih posmatrati u svojim SwiftUI prikazima radi reaktivnih ažuriranja korisničkog interfejsa (UI).

### FastCommentsSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Ukupan broj komentara na serveru |
| `newRootCommentCount` | `Int` | Privremeno sačuvani novi komentari (kada je `showLiveRightAway` false) |
| `currentUser` | `UserSessionInfo?` | Trenutno autentifikovani korisnik |
| `isSiteAdmin` | `Bool` | Da li je trenutni korisnik administrator sajta |
| `isClosed` | `Bool` | Da li je nit komentara zatvorena |
| `hasBillingIssue` | `Bool` | Da li postoji problem sa naplatom |
| `isLoading` | `Bool` | Da li je mrežni zahtev u toku |
| `hasMore` | `Bool` | Da li postoje dodatne stranice komentara |
| `blockingErrorMessage` | `String?` | Greška koja sprečava funkcionisanje korisničkog interfejsa |
| `warningMessage` | `String?` | Upozorenje koje ne blokira |
| `isDemo` | `Bool` | Da li se radi u demo režimu |
| `commentsVisible` | `Bool` | Prekidač za vidljivost komentara |
| `toolbarEnabled` | `Bool` | Da li je prikazana alatna traka za formatiranje |

### FastCommentsFeedSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Trenutno učitani postovi feeda |
| `hasMore` | `Bool` | Da li postoje dodatne stranice |
| `currentUser` | `UserSessionInfo?` | Trenutno autentifikovani korisnik |
| `blockingErrorMessage` | `String?` | Blokirajuća poruka o grešci |
| `isLoading` | `Bool` | Da li je mrežni zahtev u toku |
| `newPostsCount` | `Int` | Broj novih postova od poslednjeg učitavanja |

### Comment Tree

Stablo komentara je dostupno preko `sdk.commentsTree`:

```swift
// Ravan spisak vidljivih čvorova za prikaz
sdk.commentsTree.visibleNodes

// Pronađi komentar po ID-u
sdk.commentsTree.commentsById["comment-id"]
```

---
---