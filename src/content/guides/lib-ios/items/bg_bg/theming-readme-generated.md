### Предварителни настройки на тема

Налични са четири вградени предварителни настройки:

```swift
// Системни настройки по подразбиране
sdk.theme = FastCommentsTheme.default

// Карти с сенки и големи заоблени ъгли
sdk.theme = FastCommentsTheme.modern

// Плосък, без сенки, малък радиус на ъглите, без линии на нишката
sdk.theme = FastCommentsTheme.minimal

// Задава всички цветове за действията като един брандов цвят
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Стилове на показване на коментари

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Плосък списък с разделители (по подразбиране)
theme.commentStyle = .card    // Заоблени карти със сенки
theme.commentStyle = .bubble  // Стил тип чат балон
```

### Цветове

Всички цветови свойства са незадължителни. Непопълнените стойности се връщат към разумни системни стойности по подразбиране.

```swift
var theme = FastCommentsTheme()

// Бранд цветове
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Фонове
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Бутони за действие
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Гласувания
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Връзки
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Диалози
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Лента за въвеждане
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Други
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Типография

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Оформление и разстояния

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Точки между редовете с коментари
theme.nestingIndent = 20          // Отместване (в точки) за всяко ниво на вложеност
theme.avatarSize = 36             // Диаметър на аватара за кореновите коментари
theme.replyAvatarSize = 28        // Диаметър на аватара за вложени отговори
```

### Визуални ефекти

```swift
theme.showShadows = true          // Нежни сенки върху картите
theme.showThreadLine = true       // Вертикална линия свързваща вложените отговори
theme.animateVotes = true         // Пружинна анимация при промяна на гласовете
```

### Прилагане на теми

Два подхода:

```swift
// Чрез средата на SwiftUI (препоръчително за йерархията на изгледите)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Директно в SDK
sdk.theme = theme
```

---
---