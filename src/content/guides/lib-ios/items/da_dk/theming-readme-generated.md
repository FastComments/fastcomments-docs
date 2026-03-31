### Tema-præindstillinger

Fire indbyggede præindstillinger er tilgængelige:

```swift
// Systemstandarder
sdk.theme = FastCommentsTheme.default

// Kort med skygger og store afrundede hjørner
sdk.theme = FastCommentsTheme.modern

// Fladt, ingen skygger, lille hjørneradius, ingen trådlinjer
sdk.theme = FastCommentsTheme.minimal

// Sæt alle handlingsfarver til en enkelt brandfarve
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Kommentarvisningsstilarter

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Flad liste med skillelinjer (standard)
theme.commentStyle = .card    // Afrundede kort med skygger
theme.commentStyle = .bubble  // Chatboble-stil
```

### Farver

Alle farveegenskaber er valgfrie. Udefinerede værdier vender tilbage til fornuftige systemstandarder.

```swift
var theme = FastCommentsTheme()

// Brandfarver
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Baggrunde
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Handlingsknapper
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Stemmer
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Links
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Dialoger
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Indtastningsbjælke
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Andre
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Typografi

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Layout og mellemrum

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Punkter mellem kommentarrækker
theme.nestingIndent = 20          // Antal punkter til indrykning pr. niveau
theme.avatarSize = 36             // Avatardiameter for topniveau-kommentarer
theme.replyAvatarSize = 28        // Avatardiameter for indlejrede svar
```

### Visuelle effekter

```swift
theme.showShadows = true          // Diskrete skygger på kort
theme.showThreadLine = true       // Lodret linje, der forbinder indlejrede svar
theme.animateVotes = true         // Spring-animation ved stemmeændringer
```

### Anvendelse af temaer

To tilgange:

```swift
// Via SwiftUI-miljøet (anbefales til visningshierarki)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Direkte på SDK'en
sdk.theme = theme
```

---
---