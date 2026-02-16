---
#### Скин: Erebus
![Скин: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Скин: Default
![Скин: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Нативный WYSIWYG-редактор с поддержкой изображений!
![Нативный WYSIWYG-редактор с поддержкой изображений](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Редактор форматированного текста

Эта библиотека использует редактор 10tap для возможностей редактирования форматированного текста, который обеспечивает мощный WYSIWYG-интерфейс.

### Параметры конфигурации

Эта библиотека стремится поддерживать все параметры конфигурации, определенные в [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), так же как и веб-реализация.

### Концепции FastComments

Основные понятия, которые нужно знать, чтобы начать — это `tenantId` и `urlId`. `tenantId` — это идентификатор вашей учетной записи на FastComments.com. `urlId` — это то, к чему будут привязаны потоки комментариев. Это может быть URL страницы, идентификатор товара, идентификатор статьи и т. д.

### Уведомления пользователей

FastComments поддерживает уведомления для [многих сценариев](https://docs.fastcomments.com/guide-notifications.html). Уведомления настраиваемы, от них можно отказаться глобально или на уровне конкретного уведомления/комментария, а также поддерживаются подписки на уровне страницы, чтобы пользователи могли подписываться на потоки определенной страницы или статьи.

Например, можно использовать Secure SSO для аутентификации пользователя, а затем периодически опрашивать наличие непрочитанных уведомлений и отправлять их пользователю.

См. пример [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx), чтобы узнать, как получать и обрабатывать (переводить) непрочитанные уведомления пользователей.

### Браузер GIF

По умолчанию выбор изображений или GIF не включен. См. [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), чтобы узнать, как поддерживать загрузку изображений и GIF. В этой библиотеке есть Браузер GIF, который анонимизирует поисковые запросы и изображения, предоставляемые в библиотеке — вам просто нужно его использовать.

### Производительность

Если вы обнаружите проблемы с производительностью, пожалуйста, откройте тикет с примером для воспроизведения, включая используемое устройство. Производительность является приоритетом для всех библиотек FastComments.
---