### Préréglages de thème

Quatre préréglages intégrés sont disponibles :

```swift
// Paramètres système par défaut
sdk.theme = FastCommentsTheme.default

// Cartes avec ombres et grands coins arrondis
sdk.theme = FastCommentsTheme.modern

// Plat, sans ombres, petit rayon de coins, pas de lignes de discussion
sdk.theme = FastCommentsTheme.minimal

// Définir toutes les couleurs d'action sur une seule couleur de marque
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Styles d'affichage des commentaires

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Liste plate avec séparateurs (par défaut)
theme.commentStyle = .card    // Cartes arrondies avec ombres
theme.commentStyle = .bubble  // Style bulle de chat
```

### Couleurs

Toutes les propriétés de couleur sont optionnelles. Les valeurs non définies reviennent aux paramètres par défaut du système.

```swift
var theme = FastCommentsTheme()

// Couleurs de la marque
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Arrière-plans
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Boutons d'action
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Votes
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Liens
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Dialogues
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Barre de saisie
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Autres
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Typographie

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Mise en page et espacement

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // points entre les lignes de commentaires
theme.nestingIndent = 20          // points d'indentation par niveau d'imbrication
theme.avatarSize = 36             // diamètre de l'avatar pour les commentaires racines
theme.replyAvatarSize = 28        // diamètre de l'avatar pour les réponses imbriquées
```

### Effets visuels

```swift
theme.showShadows = true          // Ombres subtiles sur les cartes
theme.showThreadLine = true       // Ligne verticale reliant les réponses imbriquées
theme.animateVotes = true         // Animation à ressort lors des changements de vote
```

### Application des thèmes

Deux approches :

```swift
// Via l'environnement SwiftUI (recommandé pour la hiérarchie des vues)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Directement sur le SDK
sdk.theme = theme
```