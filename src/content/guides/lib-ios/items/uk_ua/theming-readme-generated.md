### Готові теми

Доступні чотири вбудовані пресети:

```swift
// Системні значення за замовчуванням
sdk.theme = FastCommentsTheme.default

// Картки з тінями та великими заокругленими кутами
sdk.theme = FastCommentsTheme.modern

// Плоский стиль, без тіней, малий радіус кутів, без ліній гілок
sdk.theme = FastCommentsTheme.minimal

// Встановити всі кольори дій в один фірмовий колір
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Стилі відображення коментарів

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Плоский список з розділювачами (за замовчуванням)
theme.commentStyle = .card    // Заокруглені картки з тінями
theme.commentStyle = .bubble  // Стиль чат-бульбашок
```

### Кольори

Усі властивості кольорів є необов'язковими. Невстановлені значення повертаються до розумних системних значень за замовчуванням.

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

// Голосування
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

// Панель введення
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
theme.commentSpacing = 4          // Відстань у пунктах між рядками коментарів
theme.nestingIndent = 20          // Відступ у пунктах на кожен рівень вкладеності
theme.avatarSize = 36             // Діаметр аватара для кореневих коментарів
theme.replyAvatarSize = 28        // Діаметр аватара для вкладених відповідей
```

### Візуальні ефекти

```swift
theme.showShadows = true          // Легкі тіні на картках
theme.showThreadLine = true       // Вертикальна лінія, що з'єднує вкладені відповіді
theme.animateVotes = true         // Пружна анімація при зміні голосів
```

### Застосування тем

Два підходи:

```swift
// Через середовище SwiftUI (рекомендовано для ієрархії представлень)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Безпосередньо на SDK
sdk.theme = theme
```

---


### Кольори опитувань

Опитування за замовчуванням використовують кольори картки опитування веб-віджета у світлому та темному режимах. Встановлення `primaryColor` також змінює акцент і смугу обраного вами варіанта. Щоб встановити їх безпосередньо:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Властивість | Використовується для |
|----------|----------|
| `pollAccentColor` | Межа та позначка обраного варіанта, обрана тривалість, "Ваш голос враховано" |
| `pollAccentTextColor` | Позначка та текст, намальовані на акцентному кольорі |
| `pollOptionBackgroundColor` | Фон чипа варіанта |
| `pollOptionPressedBackgroundColor` | Фон чипа варіанта під час натискання |
| `pollBarColor` | Смуга результату для варіантів, які ви не обрали |
| `pollSelectedBarColor` | Смуга результату для обраного вами варіанта |
| `pollBorderColor` | Позначка чекбокса та контурні пігулки |
| `pollDangerColor` | Помилки та "Видалити опитування" |