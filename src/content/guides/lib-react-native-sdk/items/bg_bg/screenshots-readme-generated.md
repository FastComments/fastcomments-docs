#### Тема: Erebus
![Тема: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Тема: Default
![Тема: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Нативен WYSIWYG редактор с поддръжка на изображения!
![Нативен WYSIWYG редактор с поддръжка на изображения](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Редактор за форматиран текст

Тази библиотека използва редактора 10tap за функционалност за редактиране на форматиран текст, който предоставя мощно WYSIWYG преживяване при редактиране.

### Опции за конфигурация

Целта на тази библиотека е да поддържа всички опции за конфигурация, дефинирани в [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), точно както уеб имплементацията.

### Основни концепции на FastComments

Основните концепции, които трябва да знаете, за да започнете, са `tenantId` и `urlId`. `tenantId` е идентификаторът на вашия акаунт в FastComments.com. `urlId` е към какво ще бъдат привързани нишките с коментари. Това може да бъде URL на страница, или идентификатор на продукт, на статия и т.н.

### Потребителски известия

FastComments поддържа известия за [много сценарии](https://docs.fastcomments.com/guide-notifications.html). Известията са конфигурируеми, потребителите могат да се откажат от тях глобално или на ниво известие/коментар, и се поддържат абонаменти на ниво страница, така че потребителите могат да се абонират за нишки на конкретна страница или статия.

Например, възможно е да се използва Secure SSO за удостоверяване на потребителя и след това периодично да се проверяват непрочетените известия и да се изпращат на потребителя.

Вижте [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за това как да получите и преведете непрочетените потребителски известия.

### Браузър за GIF

По подразбиране няма активиран избор на изображения или GIF. Вижте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за това как да поддържате качване на изображения и GIF. В тази библиотека има Браузър за GIF, който анонимизира търсенията и изображенията, просто трябва да го използвате.

### Производителност

Моля, открийте тикет с пример за възпроизвеждане, включително използваното устройство, ако установите проблеми с производителността. Производителността е приоритет във всички библиотеки на FastComments.