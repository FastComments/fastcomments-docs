### Preimpostazioni del tema

Quattro preimpostazioni integrate sono disponibili:

```swift
// Predefiniti di sistema
sdk.theme = FastCommentsTheme.default

// Schede con ombre e angoli arrotondati grandi
sdk.theme = FastCommentsTheme.modern

// Stile piatto, senza ombre, piccolo raggio degli angoli, senza linee del thread
sdk.theme = FastCommentsTheme.minimal

// Imposta tutti i colori delle azioni su un singolo colore del brand
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Stili di visualizzazione dei commenti

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Elenco piatto con divisori (predefinito)
theme.commentStyle = .card    // Schede arrotondate con ombre
theme.commentStyle = .bubble  // Stile a bolle di chat
```

### Colori

Tutte le proprietà colore sono opzionali. I valori non impostati ricadono sui valori predefiniti di sistema sensati.

```swift
var theme = FastCommentsTheme()

// Colori del brand
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Sfondi
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Pulsanti di azione
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Voti
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Link
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Dialog
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Barra di input
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Altro
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Tipografia

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Layout e spaziatura

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Punti tra le righe dei commenti
theme.nestingIndent = 20          // Punti di indentazione per ogni livello di annidamento
theme.avatarSize = 36             // Diametro dell'avatar per i commenti di primo livello
theme.replyAvatarSize = 28        // Diametro dell'avatar per le risposte annidate
```

### Effetti visivi

```swift
theme.showShadows = true          // Ombre sottili sulle schede
theme.showThreadLine = true       // Linea verticale che connette le risposte annidate
theme.animateVotes = true         // Animazione a molla sulle modifiche dei voti
```

### Applicazione dei temi

Due approcci:

```swift
// Tramite ambiente SwiftUI (consigliato per la gerarchia delle viste)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Direttamente sul SDK
sdk.theme = theme
```