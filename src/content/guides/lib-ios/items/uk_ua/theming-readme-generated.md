### Пресети тем

Доступні чотири вбудовані пресети:

```swift
// Системні налаштування за замовчуванням
sdk.theme = FastCommentsTheme.default

// Картки з тінями та великими заокругленими кутами
sdk.theme = FastCommentsTheme.modern

// Плоский, без тіней, невеликий радіус заокруглення, без ліній розгалуження
sdk.theme = FastCommentsTheme.minimal

// Встановити всі кольори дій в один фірмовий колір
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Стилі відображення коментарів

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Плоский список з роздільниками (за замовчуванням)
theme.commentStyle = .card    // Картки з заокругленими кутами та тінями
theme.commentStyle = .bubble  // Стиль чат-бульбашок
```

### Кольори

Усі властивості кольорів необов'язкові. Незадані значення використовують адекватні системні значення за замовчуванням.

```swift
var theme = FastCommentsTheme()

// Фірмові кольори
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Фони
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Кнопки дій
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Голоси
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Посилання
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Діалоги
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Панель вводу
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Інше
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Типографіка

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Макет і відступи

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Пункти між рядками коментарів
theme.nestingIndent = 20          // Відступ у пунктах на кожен рівень вкладення
theme.avatarSize = 36             // Діаметр аватара для кореневих коментарів
theme.replyAvatarSize = 28        // Діаметр аватара для вкладених відповідей
```

### Візуальні ефекти

```swift
theme.showShadows = true          // Легкі тіні на картках
theme.showThreadLine = true       // Вертикальна лінія, що з'єднує вкладені відповіді
theme.animateVotes = true         // Пружинна анімація при зміні голосів
```

### Застосування тем

Два підходи:

```swift
// Через середовище SwiftUI (рекомендується для ієрархії представлень)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Безпосередньо в SDK
sdk.theme = theme
```

---
---