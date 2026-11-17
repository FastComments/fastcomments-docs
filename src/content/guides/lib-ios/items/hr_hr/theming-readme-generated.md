### Predefinirane teme

Dostupna su četiri ugrađena preset-a:

```swift
// Sustavski zadani
sdk.theme = FastCommentsTheme.default

// Kartice sa sjenama i velikim zaobljenim kutovima
sdk.theme = FastCommentsTheme.modern

// Ravno, bez sjenâ, mali radijus kutova, bez linija niti
sdk.theme = FastCommentsTheme.minimal

// Postavlja sve boje akcija na jednu boju brenda
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Stilovi prikaza komentara

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Ravna lista s razdjelnicima (zadano)
theme.commentStyle = .card    // Zaobljene kartice sa sjenama
theme.commentStyle = .bubble  // Stil chat oblačića
```

### Boje

Sva svojstva boja su neobavezna. Nepostavljene vrijednosti vraćaju se na razumne sustavske zadane vrijednosti.

```swift
var theme = FastCommentsTheme()

// Boje brenda
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Pozadine
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Gumbi za akcije
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Glasovi
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Poveznice
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Dijalozi
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Traka za unos
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

### Raspored i razmak

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Točke između redaka komentara
theme.nestingIndent = 20          // Točke uvlačenja po razini ugniježđenja
theme.avatarSize = 36             // Promjer avatara za korijenske komentare
theme.replyAvatarSize = 28        // Promjer avatara za ugniježđene odgovore
```

### Vizualni efekti

```swift
theme.showShadows = true          // Suptilne sjene na karticama
theme.showThreadLine = true       // Okomita linija koja povezuje ugniježđene odgovore
theme.animateVotes = true         // Proljetna animacija pri promjeni glasova
```

### Primjena tema

Dva pristupa:

```swift
// Putem SwiftUI okruženja (preporučeno za hijerarhiju prikaza)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Izravno na SDK-u
sdk.theme = theme
```

---


### Boje anketa

Ankete prema zadanim postavkama koriste boje kartice ankete web widgeta, u svijetlom i tamnom načinu rada. Postavljanje `primaryColor` također mijenja naglasak i traku opcije koju ste odabrali. Da ih postavite izravno:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Svojstvo | Koristi se za |
|----------|----------|
| `pollAccentColor` | Rub i kvačicu odabrane opcije, odabrano trajanje, "Your vote is in" |
| `pollAccentTextColor` | Kvačicu i tekst nacrtan na naglasnoj boji |
| `pollOptionBackgroundColor` | Pozadinu čipa opcije |
| `pollOptionPressedBackgroundColor` | Pozadinu čipa opcije dok je pritisnut |
| `pollBarColor` | Traku rezultata za opcije koje niste odabrali |
| `pollSelectedBarColor` | Traku rezultata za opciju koju ste odabrali |
| `pollBorderColor` | Oznaku potvrdnog okvira i konturirane pilule |
| `pollDangerColor` | Pogreške i "Remove poll" |