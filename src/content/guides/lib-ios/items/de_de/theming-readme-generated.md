### Theme-Voreinstellungen

Vier integrierte Voreinstellungen sind verfügbar:

```swift
// Systemstandards
sdk.theme = FastCommentsTheme.default

// Karten mit Schatten und großen abgerundeten Ecken
sdk.theme = FastCommentsTheme.modern

// Flach, ohne Schatten, kleiner Eckradius, keine Thread-Linien
sdk.theme = FastCommentsTheme.minimal

// Alle Aktionsfarben auf eine einzige Markenfarbe setzen
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Kommentar-Anzeigestile

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Flache Liste mit Trennlinien (Standard)
theme.commentStyle = .card    // Abgerundete Karten mit Schatten
theme.commentStyle = .bubble  // Chat-Blasen-Stil
```

### Farben

Alle Farbeigenschaften sind optional. Nicht gesetzte Werte fallen auf sinnvolle Systemstandards zurück.

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

// Abstimmungen
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
theme.avatarSize = 36             // Avatar-Durchmesser für Stammkommentare
theme.replyAvatarSize = 28        // Avatar-Durchmesser für verschachtelte Antworten
```

### Visuelle Effekte

```swift
theme.showShadows = true          // Dezente Schatten auf Karten
theme.showThreadLine = true       // Vertikale Linie, die verschachtelte Antworten verbindet
theme.animateVotes = true         // Spring-Animation bei Abstimmungsänderungen
```

### Themes anwenden

Zwei Ansätze:

```swift
// Über die SwiftUI-Umgebung (empfohlen für die View-Hierarchie)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Direkt am SDK
sdk.theme = theme
```

---


### Umfrage-Farben

Umfragen übernehmen standardmäßig die Umfragekarten-Farben des Web-Widgets, im hellen und dunklen Modus. Das Setzen von `primaryColor` ändert auch die Akzentfarbe und den Balken der von Ihnen gewählten Option. Um sie direkt festzulegen:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Eigenschaft | Verwendung |
|----------|----------|
| `pollAccentColor` | Rahmen und Häkchen der ausgewählten Option, gewählte Dauer, „Ihre Stimme wurde gezählt" |
| `pollAccentTextColor` | Häkchen und Text, der auf der Akzentfarbe gezeichnet wird |
| `pollOptionBackgroundColor` | Hintergrund der Options-Chip |
| `pollOptionPressedBackgroundColor` | Hintergrund der Options-Chip beim Drücken |
| `pollBarColor` | Ergebnisbalken für Optionen, die Sie nicht gewählt haben |
| `pollSelectedBarColor` | Ergebnisbalken für die von Ihnen gewählte Option |
| `pollBorderColor` | Kontrollkästchen-Markierung und umrandete Pills |
| `pollDangerColor` | Fehler und „Umfrage entfernen" |