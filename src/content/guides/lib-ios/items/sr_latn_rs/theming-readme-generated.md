### Predlošci teme

Dostupna su četiri ugrađena predloška:

```swift
// Sistemske podrazumevane vrednosti
sdk.theme = FastCommentsTheme.default

// Kartice sa senkama i velikim zaobljenim uglovima
sdk.theme = FastCommentsTheme.modern

// Ravno, bez senki, mali radijus uglova, bez linija niti
sdk.theme = FastCommentsTheme.minimal

// Podesi sve boje akcija na jednu boju brenda
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Stilovi prikaza komentara

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Ravna lista sa separatorima (podrazumevano)
theme.commentStyle = .card    // Zaobljene kartice sa senkama
theme.commentStyle = .bubble  // Stil poruka u balončićima
```

### Boje

Sva svojstva boja su opciona. Nepodešene vrednosti vraćaju se na razumne sistemske podrazumevane vrednosti.

```swift
var theme = FastCommentsTheme()

// Boje brenda
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Pozadine
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Dugmad za akcije
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Glasovi
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Linkovi
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
theme.commentSpacing = 4          // Tačaka između redova komentara
theme.nestingIndent = 20          // Tačaka uvlačenja po nivou ugnježđivanja
theme.avatarSize = 36             // Prečnik avatara za korenske komentare
theme.replyAvatarSize = 28        // Prečnik avatara za ugnježdene odgovore
```

### Vizuelni efekti

```swift
theme.showShadows = true          // Suptilne senke na karticama
theme.showThreadLine = true       // Vertikalna linija koja povezuje ugnježdene odgovore
theme.animateVotes = true         // Animacija sa oprugom pri promenama glasova
```

### Primena tema

Dva pristupa:

```swift
// Preko SwiftUI okruženja (preporučeno za hijerarhiju prikaza)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Direktno na SDK-u
sdk.theme = theme
```

---
---