#### Тема: Erebus
![Тема: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Тема: Default
![Тема: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Нативни WYSIWYG уређивач са подршком за слике!
![Нативни WYSIWYG уређивач са подршком за слике](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Уређивач форматираног текста

Ова библиотека користи 10tap уређивач за функционалност уређивања форматираног текста, који пружа моћно WYSIWYG искуство уређивања.

### Опције конфигурације

Ова библиотека има за циљ да подржи све опције конфигурације дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), као и веб имплементација.

### Концепти FastComments

Главни концепти које треба познавати да бисте почели су `tenantId` и `urlId`. `tenantId` је идентификатор вашег FastComments.com налога. `urlId` је на шта ће бити везане нитi коментара.
То може бити URL странице, или идентификатор производа, идентификатор чланка итд.

### Обавештења корисника

FastComments подржава обавештења за [многе сценарије](https://docs.fastcomments.com/guide-notifications.html). Обавештења су конфигурисива,
могу се опт-аут-овати глобално или на нивоу обавештења/коментара, и подржавају претплате на нивоу странице тако да се корисници могу претплатити на нити одређене
странице или чланка.

На примјер, могуће је користити Secure SSO за аутентификацију корисника, а затим периодично провјеравати непрочитана обавештења и слати их кориснику.

Погледајте [пример AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) како да добијете и преведете непрочитане корисничке обавијести.

### Gif претраживач

По подразумеваној вриједности, избор слике или gif-а није омогућен. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за начин
подршке отпремања слика и gif-ова. Постоји Gif претраживач који анонимизује претраге и слике који се налазе у овој библиотеци, само га морате користити.

### Перформансе

Ако уочите било какве проблеме са перформансама, отворите тикет са примером за репродукцију, укључујући уређај који сте користили. Перформансе су приоритет
у свим FastComments библиотекама.