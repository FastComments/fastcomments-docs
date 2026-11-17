### Thema-presets

Er zijn vier ingebouwde presets beschikbaar:

```swift
// Systeemstandaarden
sdk.theme = FastCommentsTheme.default

// Kaarten met schaduwen en grote afgeronde hoeken
sdk.theme = FastCommentsTheme.modern

// Plat, geen schaduwen, kleine hoekradius, geen discussielijnen
sdk.theme = FastCommentsTheme.minimal

// Zet alle actiekleuren op één enkele merkkleur
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Weergavestijlen voor reacties

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Platte lijst met scheidingslijnen (standaard)
theme.commentStyle = .card    // Afgeronde kaarten met schaduwen
theme.commentStyle = .bubble  // Chatbubbel-stijl
```

### Kleuren

Alle kleureigenschappen zijn optioneel. Niet-ingestelde waarden vallen terug op verstandige systeemstandaarden.

```swift
var theme = FastCommentsTheme()

// Merkkleuren
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Achtergronden
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Actieknoppen
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Stemmen
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Links
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Dialogen
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Invoerbalk
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Overig
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Typografie

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Layout en spatiëring

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Punten tussen reactieregels
theme.nestingIndent = 20          // Punten inspringing per nestingsniveau
theme.avatarSize = 36             // Avatar-diameter voor hoofdreacties
theme.replyAvatarSize = 28        // Avatar-diameter voor geneste reacties
```

### Visuele effecten

```swift
theme.showShadows = true          // Subtiele schaduwen op kaarten
theme.showThreadLine = true       // Verticale lijn die geneste reacties verbindt
theme.animateVotes = true         // Spring-animatie bij stemwijzigingen
```

### Thema's toepassen

Twee benaderingen:

```swift
// Via de SwiftUI-omgeving (aanbevolen voor de view-hiërarchie)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Direct op de SDK
sdk.theme = theme
```

---


### Peilingskleuren

Peilingen gebruiken standaard de peilingskaartkleuren van de web-widget, in lichte en donkere modus. Het instellen van `primaryColor` verandert ook de accentkleur en de balk van de optie die je hebt gekozen. Om ze direct in te stellen:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Eigenschap | Gebruikt voor |
|----------|----------|
| `pollAccentColor` | Rand en vinkje van geselecteerde optie, gekozen duur, "Je stem is binnen" |
| `pollAccentTextColor` | Vinkje en tekst getekend op de accentkleur |
| `pollOptionBackgroundColor` | Achtergrond van optiechip |
| `pollOptionPressedBackgroundColor` | Achtergrond van optiechip tijdens indrukken |
| `pollBarColor` | Resultatenbalk voor opties die je niet hebt gekozen |
| `pollSelectedBarColor` | Resultatenbalk voor de optie die je hebt gekozen |
| `pollBorderColor` | Vinkje van selectievakje en omlijnde pills |
| `pollDangerColor` | Fouten en "Peiling verwijderen" |