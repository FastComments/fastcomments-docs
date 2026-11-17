### Preset dei temi

Sono disponibili quattro preset integrati:

```swift
// Impostazioni predefinite di sistema
sdk.theme = FastCommentsTheme.default

// Schede con ombre e angoli ampiamente arrotondati
sdk.theme = FastCommentsTheme.modern

// Piatto, senza ombre, raggio degli angoli piccolo, senza linee di thread
sdk.theme = FastCommentsTheme.minimal

// Imposta tutti i colori delle azioni su un singolo colore del brand
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Stili di visualizzazione dei commenti

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Elenco piatto con separatori (predefinito)
theme.commentStyle = .card    // Schede arrotondate con ombre
theme.commentStyle = .bubble  // Stile a fumetti di chat
```

### Colori

Tutte le proprietà dei colori sono opzionali. I valori non impostati ricadono su impostazioni predefinite di sistema sensate.

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

// Finestre di dialogo
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
theme.nestingIndent = 20          // Punti di rientro per livello di annidamento
theme.avatarSize = 36             // Diametro dell'avatar per i commenti principali
theme.replyAvatarSize = 28        // Diametro dell'avatar per le risposte annidate
```

### Effetti visivi

```swift
theme.showShadows = true          // Ombre sottili sulle schede
theme.showThreadLine = true       // Linea verticale che collega le risposte annidate
theme.animateVotes = true         // Animazione a molla sulle modifiche dei voti
```

### Applicazione dei temi

Due approcci:

```swift
// Tramite l'ambiente SwiftUI (consigliato per la gerarchia delle viste)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Direttamente sull'SDK
sdk.theme = theme
```

---


### Colori dei sondaggi

I sondaggi usano per impostazione predefinita i colori della scheda sondaggio del widget web, sia in modalità chiara che scura. L'impostazione di `primaryColor` modifica anche l'accento e la barra dell'opzione scelta. Per impostarli direttamente:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Proprietà | Utilizzata per |
|----------|----------|
| `pollAccentColor` | Bordo e spunta dell'opzione selezionata, durata scelta, "Il tuo voto è stato registrato" |
| `pollAccentTextColor` | Segno di spunta e testo disegnato sul colore d'accento |
| `pollOptionBackgroundColor` | Sfondo del chip dell'opzione |
| `pollOptionPressedBackgroundColor` | Sfondo del chip dell'opzione mentre è premuto |
| `pollBarColor` | Barra dei risultati per le opzioni non scelte |
| `pollSelectedBarColor` | Barra dei risultati per l'opzione scelta |
| `pollBorderColor` | Segno della casella di controllo e pillola con contorno |
| `pollDangerColor` | Errori e "Rimuovi sondaggio" |