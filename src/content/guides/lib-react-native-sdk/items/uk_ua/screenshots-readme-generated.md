#### Тема: Erebus
![Тема: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Тема: Default
![Тема: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Нативний WYSIWYG-редактор з підтримкою зображень!
![Нативний WYSIWYG-редактор з підтримкою зображень](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Редактор форматованого тексту

Ця бібліотека використовує редактор 10tap для функціональності редагування форматованого тексту, який забезпечує потужний WYSIWYG‑досвід редагування.

### Параметри конфігурації

Ця бібліотека прагне підтримувати всі параметри конфігурації, визначені в [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), так само як і веб‑реалізація.

### Основні поняття FastComments

Головні поняття, які варто знати для початку роботи — це `tenantId` та `urlId`. `tenantId` — це ідентифікатор вашого облікового запису на FastComments.com. `urlId` визначає, до чого будуть прив'язані треди коментарів. Це може бути URL сторінки, або id продукту, id статті тощо.

### Сповіщення користувачів

FastComments підтримує сповіщення для [багатьох сценаріїв](https://docs.fastcomments.com/guide-notifications.html). Сповіщення настроювані, їх можна відключити глобально або на рівні окремого сповіщення/коментаря, також підтримується підписка на рівні сторінки, щоб користувачі могли підписуватись на треди певної сторінки або статті.

Наприклад, можна використовувати Secure SSO для аутентифікації користувача, а потім періодично опитувати наявність непрочитаних сповіщень і надсилати їх користувачу.

Див. [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx), щоб дізнатися, як отримувати та перекладати непрочитані сповіщення користувача.

### GIF-переглядач

За замовчуванням вибір зображень або GIF не увімкнений. Див. [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), щоб дізнатися, як підтримувати завантаження зображень і GIF. У цій бібліотеці є GIF-переглядач, який анонімізує пошукові запити та зображення, — вам просто потрібно його використати.

### Продуктивність

Будь ласка, відкрийте тікет з прикладом для відтворення проблеми, включно з пристроєм, якщо ви виявите проблеми з продуктивністю. Продуктивність є пріоритетом у всіх бібліотеках FastComments.