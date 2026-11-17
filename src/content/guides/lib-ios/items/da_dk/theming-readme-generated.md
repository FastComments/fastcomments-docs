### Temaforudindstillinger

Fire indbyggede forudindstillinger er tilgængelige:

```swift
// Systemstandarder
sdk.theme = FastCommentsTheme.default

// Kort med skygger og store afrundede hjørner
sdk.theme = FastCommentsTheme.modern

// Flad, ingen skygger, lille hjørneradius, ingen trådlinjer
sdk.theme = FastCommentsTheme.minimal

// Sæt alle handlingsfarver til en enkelt brandfarve
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Visningsstilarter for kommentarer

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Flad liste med skillelinjer (standard)
theme.commentStyle = .card    // Afrundede kort med skygger
theme.commentStyle = .bubble  // Chatboble-stil
```

### Farver

Alle farveegenskaber er valgfrie. Værdier, der ikke er angivet, falder tilbage til fornuftige systemstandarder.

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

// Inputlinje
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Andet
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

### Layout og afstand

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Punkter mellem kommentarrækker
theme.nestingIndent = 20          // Punkter indrykning per indlejringsniveau
theme.avatarSize = 36             // Avatar-diameter for rodkommentarer
theme.replyAvatarSize = 28        // Avatar-diameter for indlejrede svar
```

### Visuelle effekter

```swift
theme.showShadows = true          // Diskrete skygger på kort
theme.showThreadLine = true       // Lodret linje, der forbinder indlejrede svar
theme.animateVotes = true         // Fjederanimation ved stemmeændringer
```

### Anvendelse af temaer

To tilgange:

```swift
// Via SwiftUI-miljøet (anbefales til visningshierarkiet)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Direkte på SDK'en
sdk.theme = theme
```

---


### Afstemningsfarver

Afstemninger bruger som standard web-widgetens afstemningskortfarver, i lys og mørk tilstand. Indstilling af `primaryColor` ændrer også accentfarven og bjælken for den valgmulighed, du valgte. For at angive dem direkte:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Egenskab | Bruges til |
|----------|----------|
| `pollAccentColor` | Kant og flueben for valgt mulighed, valgt varighed, "Din stemme er registreret" |
| `pollAccentTextColor` | Flueben og tekst tegnet på accentfarven |
| `pollOptionBackgroundColor` | Baggrund for valgmulighedschip |
| `pollOptionPressedBackgroundColor` | Baggrund for valgmulighedschip, mens den trykkes |
| `pollBarColor` | Resultatbjælke for valgmuligheder, du ikke valgte |
| `pollSelectedBarColor` | Resultatbjælke for den valgmulighed, du valgte |
| `pollBorderColor` | Afkrydsningsmærke og konturerede piller |
| `pollDangerColor` | Fejl og "Fjern afstemning" |