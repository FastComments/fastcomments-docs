### Vnaprej pripravljene teme

Na voljo so štirje vgrajeni prednastavljeni nabori:

```swift
// Sistemski privzeti
sdk.theme = FastCommentsTheme.default

// Kartice s sencami in velikimi zaobljenimi koti
sdk.theme = FastCommentsTheme.modern

// Ravno, brez senc, majhen radij kotov, brez črt niti
sdk.theme = FastCommentsTheme.minimal

// Nastavi vse barve dejanj na eno barvo blagovne znamke
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Slogi prikaza komentarjev

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Ravni seznam z ločili (privzeto)
theme.commentStyle = .card    // Zaobljene kartice s sencami
theme.commentStyle = .bubble  // Slog klepetalnih oblačkov
```

### Barve

Vse barvne lastnosti so neobvezne. Nenastavljene vrednosti se vrnejo na smiselne sistemske privzete vrednosti.

```swift
var theme = FastCommentsTheme()

// Barve blagovne znamke
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Ozadja
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Gumbi dejanj
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

// Pogovorna okna
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Vrstica za vnos
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Drugo
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
theme.nestingIndent = 20          // Točke zamika na vsako raven gnezdenja
theme.avatarSize = 36             // Premer avatarja za korenske komentarje
theme.replyAvatarSize = 28        // Premer avatarja za ugnezdene odgovore
```

### Vizualni učinki

```swift
theme.showShadows = true          // Subtilne sence na karticah
theme.showThreadLine = true       // Navpična črta, ki povezuje ugnezdene odgovore
theme.animateVotes = true         // Pomladna animacija ob spremembi glasov
```

### Uporaba tem

Dva pristopa:

```swift
// Prek okolja SwiftUI (priporočeno za hierarhijo pogledov)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Neposredno na SDK
sdk.theme = theme
```

---


### Barve anket

Ankete privzeto uporabljajo barve kartice ankete spletne pripomočka, v svetlem in temnem načinu. Nastavitev `primaryColor` spremeni tudi poudarek in vrstico izbrane možnosti. Če želite nastaviti neposredno:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Lastnost | Uporaba |
|----------|----------|
| `pollAccentColor` | Obroba in kljukica izbrane možnosti, izbrano trajanje, "Vaš glas je zabeležen" |
| `pollAccentTextColor` | Kljukica in besedilo, narisano na poudarjeni barvi |
| `pollOptionBackgroundColor` | Ozadje oblačka možnosti |
| `pollOptionPressedBackgroundColor` | Ozadje oblačka možnosti med pritiskom |
| `pollBarColor` | Vrstica rezultatov za možnosti, ki jih niste izbrali |
| `pollSelectedBarColor` | Vrstica rezultatov za izbrano možnost |
| `pollBorderColor` | Oznaka potrditvenega polja in obrobljeni oblački |
| `pollDangerColor` | Napake in "Odstrani anketo" |