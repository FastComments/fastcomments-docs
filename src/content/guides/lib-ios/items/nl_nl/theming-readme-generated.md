### Thema-voorinstellingen

Er zijn vier ingebouwde voorinstellingen beschikbaar:

```swift
// Standaardinstellingen van het systeem
sdk.theme = FastCommentsTheme.default

// Kaarten met schaduwen en grote, afgeronde hoeken
sdk.theme = FastCommentsTheme.modern

// Vlak, geen schaduwen, kleine hoekradius, geen verticale lijn tussen geneste reacties
sdk.theme = FastCommentsTheme.minimal

// Stel alle actiekleuren in op één merkkleur
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Weergavestijlen voor reacties

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Vlakke lijst met scheidingslijnen (standaard)
theme.commentStyle = .card    // Afgeronde kaarten met schaduwen
theme.commentStyle = .bubble  // Chat-bubbelstijl
```

### Kleuren

Alle kleur-eigenschappen zijn optioneel. Niet ingestelde waarden vallen terug op zinvolle systeemstandaarden.

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

// Overige
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

### Lay-out en ruimte

```swift
theme.cornerRadius = .large       // Opties: .none, .small, .medium, .large
theme.commentSpacing = 4          // Punten tussen reactierijen
theme.nestingIndent = 20          // Punten inspringing per genest niveau
theme.avatarSize = 36             // Diameter avatar voor hoofdreacties
theme.replyAvatarSize = 28        // Diameter avatar voor geneste reacties
```

### Visuele effecten

```swift
theme.showShadows = true          // Subtiele schaduwen op kaarten
theme.showThreadLine = true       // Verticale lijn die geneste reacties verbindt
theme.animateVotes = true         // Veeranimatie bij stemwijzigingen
```

### Thema's toepassen

Twee benaderingen:

```swift
// Via SwiftUI-omgeving (aanbevolen voor de weergave-hiërarchie)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Direct op de SDK
sdk.theme = theme
```

---
---