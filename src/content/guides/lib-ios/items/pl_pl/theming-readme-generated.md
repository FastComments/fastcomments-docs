---
### Presety motywów

Dostępne są cztery wbudowane presety:

```swift
// Ustawienia domyślne systemu
sdk.theme = FastCommentsTheme.default

// Karty z cieniami i dużymi zaokrąglonymi narożnikami
sdk.theme = FastCommentsTheme.modern

// Płaski, bez cieni, mały promień narożników, bez linii wątków
sdk.theme = FastCommentsTheme.minimal

// Ustaw wszystkie kolory akcji na pojedynczy kolor marki
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Style wyświetlania komentarzy

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Lista płaska z separatorami (domyślnie)
theme.commentStyle = .card    // Zaokrąglone karty z cieniami
theme.commentStyle = .bubble  // Styl dymków czatu
```

### Kolory

Wszystkie właściwości kolorów są opcjonalne. Nieustawione wartości domyślnie użyją sensownych ustawień systemowych.

```swift
var theme = FastCommentsTheme()

// Kolory marki
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Tła
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Przyciski akcji
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Głosy
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Linki
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Okna dialogowe
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Pasek wprowadzania
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Inne
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Typografia

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Układ i odstępy

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Punkty między wierszami komentarzy
theme.nestingIndent = 20          // Punkty wcięcia na poziom zagnieżdżenia
theme.avatarSize = 36             // Średnica avatara dla komentarzy głównych
theme.replyAvatarSize = 28        // Średnica avatara dla zagnieżdżonych odpowiedzi
```

### Efekty wizualne

```swift
theme.showShadows = true          // Subtelne cienie na kartach
theme.showThreadLine = true       // Pionowa linia łącząca zagnieżdżone odpowiedzi
theme.animateVotes = true         // Animacja sprężynowa przy zmianie głosów
```

### Zastosowanie motywów

Dwa sposoby:

```swift
// Poprzez środowisko SwiftUI (zalecane dla hierarchii widoków)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Bezpośrednio w SDK
sdk.theme = theme
```

---
---