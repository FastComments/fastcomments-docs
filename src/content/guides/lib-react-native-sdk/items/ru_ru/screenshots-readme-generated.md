#### Скин: Erebus
![Скин: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Скин: Default
![Скин: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Нативный WYSIWYG-редактор с поддержкой изображений!
![Нативный WYSIWYG-редактор с поддержкой изображений](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Редактор форматированного текста

Эта библиотека использует редактор 10tap для функциональности редактирования форматированного текста, который обеспечивает мощный WYSIWYG-опыт редактирования.

### Параметры конфигурации

Эта библиотека стремится поддерживать все параметры конфигурации, определённые в [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), так же как и веб-реализация.

### Концепции FastComments

Основные концепции, которые нужно понимать для начала работы — это `tenantId` и `urlId`. `tenantId` — это идентификатор вашей учётной записи FastComments.com. `urlId` — это то, к чему будут привязаны потоки комментариев. Это может быть URL страницы, идентификатор продукта, идентификатор статьи и т. д.

### Уведомления пользователей

FastComments поддерживает уведомления для [многих сценариев](https://docs.fastcomments.com/guide-notifications.html). Уведомления настраиваются, можно отказаться от них глобально или на уровне конкретного уведомления/комментария, и поддерживаются подписки на уровне страницы, чтобы пользователи могли подписываться на потоки конкретной страницы или статьи.

Например, можно использовать Secure SSO для аутентификации пользователя, а затем периодически опрашивать наличие непрочитанных уведомлений и отправлять их пользователю.

См. [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx), чтобы узнать, как получать и переводить непрочитанные пользовательские уведомления.

### Браузер GIF

По умолчанию выбор изображений или GIF не включён. См. [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), чтобы узнать, как поддерживать загрузку изображений и GIF. В этой библиотеке есть Браузер GIF, который анонимизирует поисковые запросы и изображения, предоставляемые в нём, — вам просто нужно его использовать.

### Производительность

Пожалуйста, откройте тикет с примером для воспроизведения, включая устройство, если вы обнаружите какие-либо проблемы с производительностью. Производительность является приоритетом во всех библиотеках FastComments.