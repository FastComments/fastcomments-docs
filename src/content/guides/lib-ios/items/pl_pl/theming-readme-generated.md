### Gotowe motywy

Dostępne są cztery wbudowane motywy:

```swift
// Ustawienia domyślne systemu
sdk.theme = FastCommentsTheme.default

// Karty z cieniami i dużymi zaokrąglonymi rogami
sdk.theme = FastCommentsTheme.modern

// Płaski styl, bez cieni, mały promień zaokrąglenia, bez linii wątków
sdk.theme = FastCommentsTheme.minimal

// Ustawia wszystkie kolory akcji na jeden kolor marki
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Style wyświetlania komentarzy

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Płaska lista z separatorami (domyślny)
theme.commentStyle = .card    // Zaokrąglone karty z cieniami
theme.commentStyle = .bubble  // Styl dymków czatu
```

### Kolory

Wszystkie właściwości kolorów są opcjonalne. Nieustawione wartości powracają do rozsądnych domyślnych ustawień systemu.

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
theme.avatarSize = 36             // Średnica awatara dla komentarzy głównych
theme.replyAvatarSize = 28        // Średnica awatara dla zagnieżdżonych odpowiedzi
```

### Efekty wizualne

```swift
theme.showShadows = true          // Subtelne cienie na kartach
theme.showThreadLine = true       // Pionowa linia łącząca zagnieżdżone odpowiedzi
theme.animateVotes = true         // Animacja sprężynowa przy zmianach głosów
```

### Stosowanie motywów

Dwa podejścia:

```swift
// Przez środowisko SwiftUI (zalecane dla hierarchii widoków)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Bezpośrednio na SDK
sdk.theme = theme
```

---


### Kolory ankiet

Ankiety domyślnie używają kolorów karty ankiety z widżetu webowego, w trybie jasnym i ciemnym. Ustawienie `primaryColor` zmienia również kolor akcentu oraz pasek wybranej opcji. Aby ustawić je bezpośrednio:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Właściwość | Zastosowanie |
|----------|----------|
| `pollAccentColor` | Obramowanie i zaznaczenie wybranej opcji, wybrany czas trwania, "Twój głos został oddany" |
| `pollAccentTextColor` | Znak zaznaczenia i tekst rysowany na kolorze akcentu |
| `pollOptionBackgroundColor` | Tło opcji |
| `pollOptionPressedBackgroundColor` | Tło opcji podczas naciśnięcia |
| `pollBarColor` | Pasek wyników dla opcji, których nie wybrano |
| `pollSelectedBarColor` | Pasek wyników dla wybranej opcji |
| `pollBorderColor` | Znak pola wyboru i konturowe pigułki |
| `pollDangerColor` | Błędy i "Usuń ankietę" |