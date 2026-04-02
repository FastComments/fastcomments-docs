### Предустановки темы

Доступны четыре встроенные предустановки:

```swift
// Системные настройки по умолчанию
sdk.theme = FastCommentsTheme.default

// Карточки с тенями и крупным скруглением углов
sdk.theme = FastCommentsTheme.modern

// Плоский, без теней, небольшой радиус скругления, без линий ветвления
sdk.theme = FastCommentsTheme.minimal

// Установить все цвета действий в один фирменный цвет
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Стиль отображения комментариев

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Плоский список с разделителями (по умолчанию)
theme.commentStyle = .card    // Скруглённые карточки с тенями
theme.commentStyle = .bubble  // Стиль чат-пузырьков
```

### Цвета

Все свойства цвета необязательны. Если значение не задано, используется разумный системный вариант по умолчанию.

```swift
var theme = FastCommentsTheme()

// Фирменные цвета
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Фоны
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Кнопки действий
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Голоса
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Ссылки
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Диалоги
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Поле ввода
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Прочее
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
theme.cornerRadius = .large       // варианты: .none, .small, .medium, .large
theme.commentSpacing = 4          // Пункты между строками комментариев
theme.nestingIndent = 20          // Пункты отступа на уровень вложенности
theme.avatarSize = 36             // Диаметр аватара для корневых комментариев
theme.replyAvatarSize = 28        // Диаметр аватара для вложенных ответов
```

### Визуальные эффекты

```swift
theme.showShadows = true          // Тонкие тени на карточках
theme.showThreadLine = true       // Вертикальная линия, соединяющая вложенные ответы
theme.animateVotes = true         // Пружинная анимация при изменении голосов
```

### Применение тем

Два подхода:

```swift
// Через окружение SwiftUI (рекомендуется для иерархии представлений)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Напрямую в SDK
sdk.theme = theme
```

---
---