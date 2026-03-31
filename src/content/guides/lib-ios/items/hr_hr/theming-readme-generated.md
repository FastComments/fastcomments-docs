### Predlošci tema

Dostupna su četiri ugrađena predloška:

```swift
// Zadane sistemske vrijednosti
sdk.theme = FastCommentsTheme.default

// Kartice sa sjenama i velikim zaobljenim kutovima
sdk.theme = FastCommentsTheme.modern

// Ravno, bez sjena, mali radijus uglova, bez linija niti
sdk.theme = FastCommentsTheme.minimal

// Postavi sve boje akcija na jednu boju brenda
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Stilovi prikaza komentara

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Ravna lista s razdjelnicima (zadano)
theme.commentStyle = .card    // Zaobljene kartice sa sjenama
theme.commentStyle = .bubble  // Stil razgovornih mjehurića
```

### Boje

Sva svojstva boja su opcionalna. Nepostavljene vrijednosti vraćaju se na razumne zadane vrijednosti sustava.

```swift
var theme = FastCommentsTheme()

// Boje brenda
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Pozadine
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Akcijski gumbi
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Glasovi
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Poveznice
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Dijalozi
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Traka za unos
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Ostalo
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Tipografija

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Raspored i razmaci

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Bodova između redova komentara
theme.nestingIndent = 20          // Bodova uvlačenja po razini ugnježđivanja
theme.avatarSize = 36             // Promjer avatara za komentare najviše razine
theme.replyAvatarSize = 28        // Promjer avatara za ugnježdene odgovore
```

### Vizualni efekti

```swift
theme.showShadows = true          // Diskretne sjene na karticama
theme.showThreadLine = true       // Vertikalna linija koja povezuje ugnježdene odgovore
theme.animateVotes = true         // Animacija s efektom opruge pri promjeni glasova
```

### Primjena tema

Dva pristupa:

```swift
// Putem SwiftUI okruženja (preporučeno za hijerarhiju prikaza)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Izravno na SDK
sdk.theme = theme
```

---
---