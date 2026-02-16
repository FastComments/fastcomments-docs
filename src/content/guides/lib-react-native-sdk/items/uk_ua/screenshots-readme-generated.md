#### Оформлення: Erebus
![Оформлення: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Оформлення: За замовчуванням
![Оформлення: За замовчуванням](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Нативний WYSIWYG-редактор із підтримкою зображень!
![Нативний WYSIWYG-редактор із підтримкою зображень](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Редактор форматованого тексту

Ця бібліотека використовує редактор 10tap для функціоналу редагування форматованого тексту, що забезпечує потужний WYSIWYG-досвід редагування.

### Параметри конфігурації

Ця бібліотека прагне підтримувати всі параметри конфігурації, визначені в [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), так само як і веб-реалізація.

### Поняття FastComments

Основні поняття, які потрібно знати для початку роботи — це `tenantId` та `urlId`. `tenantId` — це ідентифікатор вашого облікового запису на FastComments.com. `urlId` — це те, до чого будуть прив'язані потоки коментарів. Це може бути URL сторінки, або id продукту, id статті тощо.

### Сповіщення користувачів

FastComments підтримує сповіщення для [багатьох сценаріїв](https://docs.fastcomments.com/guide-notifications.html). Сповіщення налаштовуються,
від них можна відмовитися глобально або на рівні конкретного сповіщення/коментаря, і підтримуються підписки на рівні сторінки, щоб користувачі могли підписатися на потоки коментарів певної сторінки чи статті.

Наприклад, можна використовувати Secure SSO для автентифікації користувача, а потім періодично опитувати непрочитані сповіщення та надсилати їх користувачу.

Див. [приклад AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx), щоб дізнатися, як отримувати та перекладати непрочитані сповіщення користувача.

### Переглядач GIF

За замовчуванням вибір зображень або GIF не активовано. Див. [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), щоб дізнатися, як підтримати завантаження зображень і GIF. У цій бібліотеці є Переглядач GIF, який анонімізує пошукові запити та надані зображення — потрібно лише його використовувати.

### Продуктивність

Будь ласка, відкрийте тікет із прикладом для відтворення, включно з інформацією про пристрій, якщо ви виявите будь-які проблеми з продуктивністю. Продуктивність має високий пріоритет у всіх бібліотеках FastComments.