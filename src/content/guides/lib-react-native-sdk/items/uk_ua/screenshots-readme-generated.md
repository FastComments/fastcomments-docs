#### Skin: Erebus
![Тема: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Skin: Default
![Тема: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Native WYSIWYG Editor with Image Support!
![Нативний WYSIWYG-редактор з підтримкою зображень](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Rich Text Editor

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native, and a bundler that resolves package `exports` conditions (Metro with package exports / RN 0.72+). Web support is currently experimental.

### Configuration Options

This library aims to support all configuration options defined in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), just like the web implementation.

### FastComments Concepts

Основні поняття, які потрібно знати для початку роботи — це `tenantId` та `urlId`. `tenantId` — це ідентифікатор вашого облікового запису на FastComments.com. `urlId` — це прив’язка, до якої будуть прикріплені потоки коментарів. Це може бути URL сторінки, ідентифікатор продукту, ідентифікатор статті тощо.

### User Notifications

FastComments supports notifications for [many scenarios](https://docs.fastcomments.com/guide-notifications.html). Сповіщення налаштовуються, можна відмовитися від них глобально або на рівні конкретного сповіщення/коментаря, а також підтримуються підписки на рівні сторінки, щоб користувачі могли підписуватися на потоки конкретної сторінки або статті.

Наприклад, можна використовувати Secure SSO для аутентифікації користувача, а потім періодично опитувати на наявність непрочитаних сповіщень і надсилати їх користувачу.

Дивіться [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx), щоб дізнатися, як отримувати та перекладати непрочитані сповіщення користувача.

### Gif Browser

За замовчуванням вибір зображень або GIF не увімкнено. Дивіться [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), щоб дізнатися, як підтримувати завантаження зображень і GIF. У цій бібліотеці є переглядач GIF, який анонімізує пошуки та зображення — вам потрібно лише його використати.

### Performance

Будь ласка, відкрийте тікет із прикладом для відтворення, включаючи використаний пристрій, якщо ви виявите проблеми з продуктивністю. Продуктивність є пріоритетом для всіх бібліотек FastComments.