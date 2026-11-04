I `FastCommentsSDK` i `FastCommentsFeedSDK` su `ObservableObject` klase sa `@Published` svojstvima. Možete ih posmatrati u vašim SwiftUI prikazima za reaktivna ažuriranja UI-ja.

### FastCommentsSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Ukupan broj komentara na serveru |
| `newRootCommentCount` | `Int` | Baferovani novi komentari (kada je `showLiveRightAway` false) |
| `currentUser` | `UserSessionInfo?` | Trenutni autentifikovani korisnik |
| `isSiteAdmin` | `Bool` | Da li je trenutni korisnik administrator sajta |
| `isClosed` | `Bool` | Da li je nit komentara zatvorena |
| `hasBillingIssue` | `Bool` | Da li postoji problem sa naplatom |
| `isLoading` | `Bool` | Da li je mrežni zahtev u toku |
| `hasMore` | `Bool` | Da li postoje još stranice komentara |
| `blockingErrorMessage` | `String?` | Greška koja sprečava funkcionisanje UI-ja |
| `warningMessage` | `String?` | Neprijavljena upozoravajuća poruka koja ne blokira |
| `isDemo` | `Bool` | Da li radi u demo režimu |
| `commentsVisible` | `Bool` | Prekidač za vidljivost komentara |
| `toolbarEnabled` | `Bool` | Da li je prikazana traka sa alatima za formatiranje |

### FastCommentsFeedSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Trenutno učitani objave u feedu |
| `hasMore` | `Bool` | Da li postoje još stranice |
| `currentUser` | `UserSessionInfo?` | Trenutni autentifikovani korisnik |
| `blockingErrorMessage` | `String?` | Poruka greške koja blokira |
| `isLoading` | `Bool` | Da li je mrežni zahtev u toku |
| `newPostsCount` | `Int` | Broj novih objava od poslednjeg učitavanja |

### Stablo komentara

Stablu komentara se može pristupiti preko `sdk.commentsTree`:

```swift
// Flat list of visible nodes for rendering
sdk.commentsTree.visibleNodes

// Lookup a comment by ID
sdk.commentsTree.commentsById["comment-id"]
```

---