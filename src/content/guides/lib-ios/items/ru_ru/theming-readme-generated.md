### Предустановки темы

Доступны четыре встроенные предустановки:

```swift
// System defaults
sdk.theme = FastCommentsTheme.default

// Cards with shadows and large rounded corners
sdk.theme = FastCommentsTheme.modern

// Flat, no shadows, small corner radius, no thread lines
sdk.theme = FastCommentsTheme.minimal

// Set all action colors to a single brand color
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Стили отображения комментариев

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Flat list with dividers (default)
theme.commentStyle = .card    // Rounded cards with shadows
theme.commentStyle = .bubble  // Chat bubble style
```

### Цвета

Все свойства цвета необязательны. Незаполненные значения возвращаются к разумным системным значениям по умолчанию.

```swift
var theme = FastCommentsTheme()

// Brand colors
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Backgrounds
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Action buttons
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Votes
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Links
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Dialogs
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Input bar
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Other
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Типографика

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Макет и отступы

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Точек между строками комментариев
theme.nestingIndent = 20          // Отступ в точках на уровень вложенности
theme.avatarSize = 36             // Диаметр аватара для корневых комментариев
theme.replyAvatarSize = 28        // Диаметр аватара для вложенных ответов
```

### Визуальные эффекты

```swift
theme.showShadows = true          // Subtle shadows on cards
theme.showThreadLine = true       // Vertical line connecting nested replies
theme.animateVotes = true         // Spring animation on vote changes
```

### Применение тем

Два подхода:

```swift
// Via SwiftUI environment (recommended for view hierarchy)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Directly on the SDK
sdk.theme = theme
```

---
---