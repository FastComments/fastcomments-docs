### Унапред дефинисане теме

Доступне су четири уграђене преднаподешене теме:

```swift
// Системске подразумеване вредности
sdk.theme = FastCommentsTheme.default

// Картице са сенкама и великим заобљеним угловима
sdk.theme = FastCommentsTheme.modern

// Плоснато, без сенки, мали заобљени угао, без линија нити
sdk.theme = FastCommentsTheme.minimal

// Постави све боје акција на једну боју бренда
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Стилови приказа коментара

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Плосната листа са раздвајачима (подразумевано)
theme.commentStyle = .card    // Заобљене картице са сенкама
theme.commentStyle = .bubble  // Стил балона за ћаскање
```

### Боје

Сва својства боја су опциони. Ако нису подешена, користе се разумне системске подразумеване вредности.

```swift
var theme = FastCommentsTheme()

// Боје бренда
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Позадине
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Дугмад за акције
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Гласови
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Везе
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Дијалози
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Трака за унос
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Остало
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Типографија

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Распоред и размак

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Тачке између редова коментара
theme.nestingIndent = 20          // Тачке увлачења по нивоу угнежђавања
theme.avatarSize = 36             // Дијаметар аватара за коренске коментаре
theme.replyAvatarSize = 28        // Дијаметар аватара за угнежђене одговоре
```

### Визуелни ефекти

```swift
theme.showShadows = true          // Нежне сенке на картицама
theme.showThreadLine = true       // Вертикална линија која повезује угнежђене одговоре
theme.animateVotes = true         // Еластична анимација при променама гласа
```

### Примена тема

Два приступа:

```swift
// Via SwiftUI environment (recommended for view hierarchy)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Directly on the SDK
sdk.theme = theme
```