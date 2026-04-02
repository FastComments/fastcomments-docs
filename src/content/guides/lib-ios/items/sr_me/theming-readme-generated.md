### Предлошци тема

Четири уграђена пресета су доступна:

```swift
// Системске подразумеване вредности
sdk.theme = FastCommentsTheme.default

// Картице са сенкама и великим заобљеним угловима
sdk.theme = FastCommentsTheme.modern

// Раван стил, без сенки, мали радијус угла, без линија нити
sdk.theme = FastCommentsTheme.minimal

// Постави све боје акција на једну боју бренда
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Стилови приказа коментара

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Раван списак са разделницама (подразумевано)
theme.commentStyle = .card    // Заобљене картице са сенкама
theme.commentStyle = .bubble  // Стил разговора (балончић)
```

### Боје

Сва својства боја су опциони. Непостављене вредности враћају се на смислене подразумеване вредности система.

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

// Гласања
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Линкови
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Дијалози
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Улазна трака
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
theme.nestingIndent = 20          // Тачке увлачења по нивоу угнежђивања
theme.avatarSize = 36             // Пречник аватара за коренске коментаре
theme.replyAvatarSize = 28        // Пречник аватара за угнеждене одговоре
```

### Визуелни ефекти

```swift
theme.showShadows = true          // Нежне сенке на картицама
theme.showThreadLine = true       // Вертикална линија која повезује угнеждене одговоре
theme.animateVotes = true         // Анимација са пружином при променама гласа
```

### Примена тема

Два приступа:

```swift
// Преко SwiftUI окружења (препоручено за хијерархију приказа)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Директно на SDK
sdk.theme = theme
```

---
---