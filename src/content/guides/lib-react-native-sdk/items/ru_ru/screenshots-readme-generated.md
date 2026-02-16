#### Оформление: Erebus
![Оформление: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Оформление: Default
![Оформление: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Нативный WYSIWYG-редактор с поддержкой изображений!
![Нативный WYSIWYG-редактор с поддержкой изображений](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Редактор форматированного текста

Эта библиотека использует редактор 10tap для функциональности редактирования форматированного текста, который обеспечивает мощный WYSIWYG-интерфейс редактирования.

### Параметры конфигурации

Эта библиотека стремится поддерживать все параметры конфигурации, определённые в [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), так же как и веб-реализация.

### Основные понятия FastComments

Основные понятия, которые нужно знать для начала работы — это `tenantId` и `urlId`. `tenantId` — это идентификатор вашей учётной записи на FastComments.com. `urlId` — это то, к чему будут привязаны ветки комментариев. Это может быть URL страницы, идентификатор товара, идентификатор статьи и т.д.

### Уведомления пользователей

FastComments поддерживает уведомления для [многих сценариев](https://docs.fastcomments.com/guide-notifications.html). Уведомления настраиваются, могут быть отключены глобально или на уровне отдельного уведомления/комментария, и поддерживают подписки на уровне страницы, чтобы пользователи могли подписываться на обсуждения конкретной страницы или статьи.

Например, можно использовать Secure SSO для аутентификации пользователя, а затем периодически опрашивать наличие непрочитанных уведомлений и отправлять их пользователю.

Смотрите [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx), чтобы узнать, как получить и перевести непрочитанные уведомления пользователя.

### Браузер GIF

По умолчанию выбор изображений или GIF не включён. См. [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), чтобы узнать, как поддерживать загрузку изображений и GIF. В библиотеке есть Браузер GIF, который анонимизирует поисковые запросы и изображения — вам просто нужно его использовать.

### Производительность

Пожалуйста, откройте тикет с примером для воспроизведения, включая используемое устройство, если вы обнаружите проблемы с производительностью. Производительность является приоритетом во всех библиотеках FastComments.