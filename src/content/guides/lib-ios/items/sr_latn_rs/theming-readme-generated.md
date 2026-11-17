### Predefinisane teme

Dostupna su četiri ugrađena preset-a:

```swift
// Sistemski podrazumevani
sdk.theme = FastCommentsTheme.default

// Kartice sa senkama i velikim zaobljenim uglovima
sdk.theme = FastCommentsTheme.modern

// Ravan dizajn, bez senki, mali radijus uglova, bez linija niti
sdk.theme = FastCommentsTheme.minimal

// Postavlja sve boje akcija na jednu boju brenda
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Stilovi prikaza komentara

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Ravna lista sa razdelnicima (podrazumevano)
theme.commentStyle = .card    // Zaobljene kartice sa senkama
theme.commentStyle = .bubble  // Stil u obliku ćavrljanja
```

### Boje

Sve boje su opciona svojstva. Nepostavljene vrednosti se vraćaju na razumne sistemske podrazumevane vrednosti.

```swift
var theme = FastCommentsTheme()

// Boje brenda
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Pozadine
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Dugmad za akcije
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Glasovi
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Linkovi
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

### Raspored i razmaci

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Poeni između redova komentara
theme.nestingIndent = 20          // Poeni uvlačenja po nivou ugnježđivanja
theme.avatarSize = 36             // Prečnik avatara za root komentare
theme.replyAvatarSize = 28        // Prečnik avatara za ugnježdene odgovore
```

### Vizuelni efekti

```swift
theme.showShadows = true          // Suptilne senke na karticama
theme.showThreadLine = true       // Vertikalna linija koja povezuje ugnježdene odgovore
theme.animateVotes = true         // Spring animacija pri promeni glasova
```

### Primenjivanje tema

Dva pristupa:

```swift
// Preko SwiftUI okruženja (preporučeno za hijerarhiju prikaza)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Direktno na SDK
sdk.theme = theme
```

---


### Boje anketa

Ankete podrazumevano koriste boje kartice ankete sa web vidžeta, u svetlom i tamnom režimu. Postavljanje `primaryColor` takođe menja akcenat i traku opcije koju ste izabrali. Da ih postavite direktno:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Svojstvo | Koristi se za |
|----------|----------|
| `pollAccentColor` | Ivicu i čekmark izabrane opcije, izabrano trajanje, "Vaš glas je zabeležen" |
| `pollAccentTextColor` | Čekmark i tekst nacrtan na akcentnoj boji |
| `pollOptionBackgroundColor` | Pozadinu čipa opcije |
| `pollOptionPressedBackgroundColor` | Pozadinu čipa opcije dok je pritisnut |
| `pollBarColor` | Traku rezultata za opcije koje niste izabrali |
| `pollSelectedBarColor` | Traku rezultata za opciju koju ste izabrali |
| `pollBorderColor` | Oznaku polja za potvrdu i zaobljene pilule sa ivicom |
| `pollDangerColor` | Greške i "Ukloni anketu" |