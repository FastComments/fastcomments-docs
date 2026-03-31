### Theme-Voreinstellungen

Vier integrierte Voreinstellungen sind verfügbar:

```swift
// System-Standardeinstellungen
sdk.theme = FastCommentsTheme.default

// Karten mit Schatten und großen abgerundeten Ecken
sdk.theme = FastCommentsTheme.modern

// Flach, keine Schatten, kleine Eckradien, keine Thread-Linien
sdk.theme = FastCommentsTheme.minimal

// Setzt alle Aktionsfarben auf eine einzelne Markenfarbe
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Anzeigeformate für Kommentare

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Flache Liste mit Trennlinien (Standard)
theme.commentStyle = .card    // Abgerundete Karten mit Schatten
theme.commentStyle = .bubble  // Chatblasen-Stil
```

### Farben

Alle Farb-Eigenschaften sind optional. Nicht festgelegte Werte fallen auf sinnvolle Systemstandardwerte zurück.

```swift
var theme = FastCommentsTheme()

// Markenfarben
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Hintergründe
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Aktionsschaltflächen
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Stimmen
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Links
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Dialoge
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Eingabeleiste
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Sonstiges
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

### Layout und Abstände

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Punkte zwischen Kommentarzeilen
theme.nestingIndent = 20          // Punkte Einrückung pro Verschachtelungsebene
theme.avatarSize = 36             // Avatar-Durchmesser für oberste Kommentare
theme.replyAvatarSize = 28        // Avatar-Durchmesser für verschachtelte Antworten
```

### Visuelle Effekte

```swift
theme.showShadows = true          // Dezente Schatten auf Karten
theme.showThreadLine = true       // Vertikale Linie, die verschachtelte Antworten verbindet
theme.animateVotes = true         // Federanimation bei Stimmenänderungen
```

### Themes anwenden

Zwei Ansätze:

```swift
// Über die SwiftUI-Umgebung (für die View-Hierarchie empfohlen)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Direkt im SDK
sdk.theme = theme
```

---
---