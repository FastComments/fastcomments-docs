#### Тема: Erebus
![Тема: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Тема: Default
![Тема: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Нативен WYSIWYG редактор с поддръжка на изображения!
![Нативен WYSIWYG редактор с поддръжка на изображения](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Редактор за форматиран текст

Тази библиотека използва [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) за редактиране на форматиран текст, което осигурява мощно WYSIWYG преживяване при редактиране. Същият редактор задвижва iOS, Android и уеб (чрез `react-native-web`), така че композиторът се държи последователно на всяка платформа с една-единствена имплементация.

`react-native-enriched` изисква React Native New Architecture (Fabric) на нативните платформи, и bundler, който решава условията за package `exports` (Metro with package exports / RN 0.72+). Поддръжката за уеб в момента е експериментална.

### Опции за конфигурация

Тази библиотека се стреми да поддържа всички опции за конфигурация, дефинирани в [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), по същия начин както уеб имплементацията.

### Концепции на FastComments

Основните концепции, които трябва да разберете, за да започнете, са `tenantId` и `urlId`. `tenantId` е вашият идентификатор на акаунт в FastComments.com. `urlId` е мястото, към което ще бъдат свързани нишките с коментари.
Това може да бъде URL на страница, id на продукт, id на статия и т.н.

### Потребителски известия

FastComments поддържа известия за [много сценарии](https://docs.fastcomments.com/guide-notifications.html). Известията са конфигурируеми,
могат да бъдат отказани глобално или на ниво известие/коментар, и поддържат абонаменти на ниво страница, така че потребителите могат да се абонират за нишки от
конкретна страница или статия.

Например, възможно е да се използва Secure SSO за удостоверяване на потребителя и след това периодично да се проверяват непрочетени известия и да се изпращат на потребителя.

Вижте [пример AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за това как да получите и преведете непрочетени потребителски известия.

### Gif браузър

По подразбиране не е активиран избор на изображения или GIF. Вижте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за начина на поддръжка на качване на изображения и GIF. В тази библиотека има GIF браузър, който анонимизира търсенията и изображенията, просто трябва да го използвате.

### Производителност

Моля, отворете тикет с пример за възпроизвеждане, включително използваното устройство, ако установите проблеми с производителността. Производителността е приоритет във всички библиотеки на FastComments.