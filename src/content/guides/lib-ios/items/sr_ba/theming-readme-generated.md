### Преднаподешавања тема

Доступна су четири унапред дефинисана пресета:

```swift
// Системске подразумеване вриједности
sdk.theme = FastCommentsTheme.default

// Картице са сенкама и великим заобљеним ивицама
sdk.theme = FastCommentsTheme.modern

// Равне, без сенки, мали радијус ивица, без линија нити
sdk.theme = FastCommentsTheme.minimal

// Подеси све боје акција на једну боју бренда
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Стилови приказа коментара

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Равна листа са раздвајачима (подразумевано)
theme.commentStyle = .card    // Заобљене картице са сенкама
theme.commentStyle = .bubble  // Стил балонића
```

### Боје

Сва својства боја су опционална. Непостављене вриједности враћају се на разумне системске подразумеване вриједности.

```swift
var theme = FastCommentsTheme()

// Боје бренда
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Позадине
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Дугмићи акција
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Гласови
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
theme.cornerRadius = .large       // Опције: .none, .small, .medium, .large
theme.commentSpacing = 4          // Тачке између редова коментара
theme.nestingIndent = 20          // Тачке увлачења по нивоу уградње
theme.avatarSize = 36             // Пречник аватара за коренске коментаре
theme.replyAvatarSize = 28        // Пречник аватара за унутрашње одговоре
```

### Визуелни ефекти

```swift
theme.showShadows = true          // Благе сенке на картицама
theme.showThreadLine = true       // Вертикална линија која повезује уграђене одговоре
theme.animateVotes = true         // Опружна (spring) анимација при промјенама гласа
```

### Примјена тема

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