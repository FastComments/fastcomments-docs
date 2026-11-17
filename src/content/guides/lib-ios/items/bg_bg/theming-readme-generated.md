### Готови теми

Налични са четири вградени пресета:

```swift
// Системни настройки по подразбиране
sdk.theme = FastCommentsTheme.default

// Карти със сенки и големи заоблени ъгли
sdk.theme = FastCommentsTheme.modern

// Плосък стил, без сенки, малък радиус на ъглите, без линии на нишките
sdk.theme = FastCommentsTheme.minimal

// Задаване на всички цветове за действия в един цвят на марката
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Стилове за показване на коментари

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Плосък списък с разделители (по подразбиране)
theme.commentStyle = .card    // Заоблени карти със сенки
theme.commentStyle = .bubble  // Стил на чат балончета
```

### Цветове

Всички цветови свойства са по избор. Липсващите стойности се заместват с разумни системни настройки по подразбиране.

```swift
var theme = FastCommentsTheme()

// Цветове на марката
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Фонове
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Бутони за действия
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

### Оформление и отстояния

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Точки между редовете с коментари
theme.nestingIndent = 20          // Точки отстъп на ниво на влагане
theme.avatarSize = 36             // Диаметър на аватара за коренни коментари
theme.replyAvatarSize = 28        // Диаметър на аватара за вложени отговори
```

### Визуални ефекти

```swift
theme.showShadows = true          // Фини сенки върху картите
theme.showThreadLine = true       // Вертикална линия, свързваща вложените отговори
theme.animateVotes = true         // Пружинна анимация при промяна на гласуванията
```

### Прилагане на теми

Два подхода:

```swift
// Чрез SwiftUI среда (препоръчително за йерархията от изгледи)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Директно върху SDK
sdk.theme = theme
```

---


### Цветове на анкетите

Анкетите по подразбиране използват цветовете на картата за анкета от уеб уиджета, както в светъл, така и в тъмен режим. Задаването на `primaryColor` също променя акцента и лентата на избрана от вас опция. За да ги зададете директно:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Свойство | Използва се за |
|----------|----------|
| `pollAccentColor` | Рамка и отметка на избраната опция, избраната продължителност, „Вашият вот е регистриран" |
| `pollAccentTextColor` | Отметка и текст, изчертани върху акцентния цвят |
| `pollOptionBackgroundColor` | Фон на плочката на опцията |
| `pollOptionPressedBackgroundColor` | Фон на плочката на опцията при натискане |
| `pollBarColor` | Лента с резултати за опциите, които не сте избрали |
| `pollSelectedBarColor` | Лента с резултати за избраната от вас опция |
| `pollBorderColor` | Отметка на квадратчето и контурни капсули |
| `pollDangerColor` | Грешки и „Премахване на анкетата" |