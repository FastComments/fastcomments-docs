### Prednastavitve teme

Na voljo so štiri vgrajene prednastavitve:

```swift
// Sistemske privzete
sdk.theme = FastCommentsTheme.default

// Karte z zasenčenjem in velikimi zaobljenimi vogali
sdk.theme = FastCommentsTheme.modern

// Ravno, brez senc, majhen radij vogalov, brez črt niti
sdk.theme = FastCommentsTheme.minimal

// Nastavi vse barve dejanj na eno barvo znamke
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Slogi prikaza komentarjev

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Ravni seznam z ločili (privzeto)
theme.commentStyle = .card    // Zaobljene kartice s sencami
theme.commentStyle = .bubble  // Slog pogovorne oblačke
```

### Barve

Vse lastnosti barv so neobvezne. Nenastavljene vrednosti se vrnejo na smiselne sistemske privzete vrednosti.

```swift
var theme = FastCommentsTheme()

// Barve znamke
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Ozadja
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Gumbi za dejanja
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Glasovi
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Povezave
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Dialogi
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Vnosna vrstica
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

### Postavitev in razmiki

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Točke med vrsticami komentarjev
theme.nestingIndent = 20          // Točke zamika za vsako raven zagnieždenja
theme.avatarSize = 36             // Premer avatarja za korenske komentarje
theme.replyAvatarSize = 28        // Premer avatarja za gnezdene odgovore
```

### Vizualni učinki

```swift
theme.showShadows = true          // Nežne sence na karticah
theme.showThreadLine = true       // Navpična črta, ki povezuje gnezdene odgovore
theme.animateVotes = true         // Vzmetna animacija ob spremembah glasov
```

### Uporaba tem

Dva pristopa:

```swift
// Preko SwiftUI okolja (priporočeno za hierarhijo pogledov)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Neposredno na SDK
sdk.theme = theme
```

---
---