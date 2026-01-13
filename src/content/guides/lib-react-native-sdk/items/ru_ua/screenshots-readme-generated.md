#### Скин: Erebus
![Скин: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Скин: Default
![Скин: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Нативный WYSIWYG-редактор с поддержкой изображений!
![Нативный WYSIWYG-редактор с поддержкой изображений](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Редактор форматированного текста

Эта библиотека использует редактор 10tap для работы с форматированным текстом, который обеспечивает мощный WYSIWYG-инструмент редактирования.

### Параметры конфигурации

Эта библиотека стремится поддерживать все параметры конфигурации, определённые в [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), так же как и веб-реализация.

### Концепции FastComments

Основные понятия, с которыми следует ознакомиться для начала работы, — это `tenantId` и `urlId`. `tenantId` — это идентификатор вашей учётной записи на FastComments.com. `urlId` — место, к которому будут привязаны потоки комментариев. Это может быть URL страницы, идентификатор товара, идентификатор статьи и т.д.

### Уведомления пользователей

FastComments поддерживает уведомления для [множества сценариев](https://docs.fastcomments.com/guide-notifications.html). Уведомления настраиваются, от них можно отказаться глобально или на уровне отдельного уведомления/комментария, а также поддерживается подписка на уровне страницы, позволяющая пользователям подписываться на потоки определённой страницы или статьи.

Например, можно использовать Secure SSO для аутентификации пользователя, а затем периодически опрашивать наличие непрочитанных уведомлений и отправлять их пользователю.

См. [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) о том, как получать и переводить непрочитанные уведомления пользователей.

### Браузер GIF

По умолчанию выбор изображений или GIF не включён. См. [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) о том, как поддержать загрузку изображений и GIF. В этой библиотеке есть браузер GIF, который анонимизирует поисковые запросы и предоставленные изображения — вам просто нужно его использовать.

### Производительность

Если вы обнаружите проблемы с производительностью, пожалуйста, откройте тикет с примером для воспроизведения и указанием используемого устройства. Производительность является приоритетом во всех библиотеках FastComments.