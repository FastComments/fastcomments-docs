### Готовые темы

Доступны четыре встроенных пресета:

```swift
// Системные настройки по умолчанию
sdk.theme = FastCommentsTheme.default

// Карточки с тенями и большими скруглёнными углами
sdk.theme = FastCommentsTheme.modern

// Плоский стиль, без теней, малый радиус скругления, без линий веток
sdk.theme = FastCommentsTheme.minimal

// Установить все акцентные цвета в один цвет бренда
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Стили отображения комментариев

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Плоский список с разделителями (по умолчанию)
theme.commentStyle = .card    // Скруглённые карточки с тенями
theme.commentStyle = .bubble  // Стиль пузырей чата
```

### Цвета

Все свойства цветов необязательны. Неустановленные значения откатываются к разумным системным значениям по умолчанию.

```swift
var theme = FastCommentsTheme()

// Цвета бренда
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

// Панель ввода
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

### Компоновка и отступы

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Отступы между строками комментариев в пунктах
theme.nestingIndent = 20          // Отступ в пунктах на каждый уровень вложенности
theme.avatarSize = 36             // Диаметр аватара для корневых комментариев
theme.replyAvatarSize = 28        // Диаметр аватара для вложенных ответов
```

### Визуальные эффекты

```swift
theme.showShadows = true          // Лёгкие тени на карточках
theme.showThreadLine = true       // Вертикальная линия, соединяющая вложенные ответы
theme.animateVotes = true         // Пружинная анимация при изменении голосов
```

### Применение тем

Два подхода:

```swift
// Через окружение SwiftUI (рекомендуется для иерархии представлений)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Напрямую на SDK
sdk.theme = theme
```

---


### Цвета опросов

По умолчанию опросы используют цвета карточки опроса веб-виджета в светлой и тёмной темах. Установка `primaryColor` также меняет акцентный цвет и полосу выбранного вами варианта. Чтобы задать их напрямую:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Свойство | Используется для |
|----------|----------|
| `pollAccentColor` | Рамки и галочки выбранного варианта, выбранной длительности, «Ваш голос учтён» |
| `pollAccentTextColor` | Галочки и текста, нарисованного на акцентном цвете |
| `pollOptionBackgroundColor` | Фона плашки варианта |
| `pollOptionPressedBackgroundColor` | Фона плашки варианта при нажатии |
| `pollBarColor` | Полосы результата для вариантов, которые вы не выбрали |
| `pollSelectedBarColor` | Полосы результата для выбранного вами варианта |
| `pollBorderColor` | Отметки чекбокса и контурных пилюль |
| `pollDangerColor` | Ошибок и «Удалить опрос» |