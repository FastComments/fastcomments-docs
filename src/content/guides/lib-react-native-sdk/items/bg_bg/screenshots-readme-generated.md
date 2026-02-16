#### Skin: Erebus
![Skin: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Skin: Default
![Skin: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Native WYSIWYG Editor with Image Support!
![Native WYSIWYG Editor with Image Support](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Rich Text Editor

Тази библиотека използва редактора 10tap за функционалност за редактиране на богато форматиран текст, който предоставя мощно WYSIWYG изживяване при редактиране.

### Configuration Options

Тази библиотека има за цел да поддържа всички опции за конфигурация, дефинирани в [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), точно както уеб имплементацията.

### FastComments Concepts

Основните понятия, които трябва да знаете, за да започнете, са `tenantId` и `urlId`. `tenantId` е идентификаторът на вашия акаунт в FastComments.com. `urlId` е към какво ще бъдат привързани нишките с коментари. Това може да бъде URL на страница, идентификатор на продукт, идентификатор на статия и т.н.

### User Notifications

FastComments поддържа уведомления за [много сценарии](https://docs.fastcomments.com/guide-notifications.html). Уведомленията са конфигурируеми, потребителите могат да се откажат от тях глобално или на ниво уведомление/коментар, и се поддържат абонаменти на ниво страница, така че потребителите да могат да се абонират за нишки от конкретна страница или статия.

Например, възможно е да използвате Secure SSO за удостоверяване на потребителя и след това периодично да проверявате за непрочетени уведомления и да ги изпращате на потребителя.

Вижте [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за това как да получите и преведете непрочетени потребителски уведомления.

### Gif Browser

По подразбиране не е разрешен избор на изображение или GIF. Вижте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за това как да поддържате качване на изображения и GIF. В библиотеката има GIF браузър, който анонимизира търсенията и изображението, предоставено в нея — просто трябва да го използвате.

### Performance

Моля, отворете тикет с пример за възпроизвеждане, включително използваното устройство, ако установите проблеми с производителността. Производителността е основен приоритет във всички библиотеки на FastComments.