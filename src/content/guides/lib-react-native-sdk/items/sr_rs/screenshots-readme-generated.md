#### Скин: Erebus
![Скин: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Скин: Default
![Скин: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Нативни WYSIWYG уређивач са подршком за слике!
![Нативни WYSIWYG уређивач са подршком за слике](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Уређивач богатог текста

Ова библиотека користи 10tap уређивач за функционалност уређивања богатог текста, који пружа моћно WYSIWYG корисничко искуство уређивања.

### Опције конфигурације

Ова библиотека има за циљ да подржи све опције конфигурације дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), као и веб имплементација.

### Концепти FastComments

Главни концепти које треба имати у виду за почетак су `tenantId` и `urlId`. `tenantId` је идентификатор вашег FastComments.com налога. `urlId` је место на које ће бити везане нитi коментара.
Ово може бити URL странице, или ид производа, ид чланка итд.

### Обавештења за кориснике

FastComments подржава обавештења за [многе сценарије](https://docs.fastcomments.com/guide-notifications.html). Обавештења су конфигурисана,
могуће је одјавити се глобално или на нивоу обавештења/коментара, и подржавају претплате на нивоу странице тако да се корисници могу претплатити на нит и
конкретне странице или чланке.

На пример, могуће је користити Secure SSO за аутентификацију корисника и затим периодично проверавати непрочитана обавештења и прослеђивати их кориснику.

Погледајте [пример AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за начин како добити и превести непрочитана корисничка обавештења.

### Гиф прегледач

Подразумевано није омогућен избор слика или гифова. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за начин
како подржати отпремање слика и гифова. У овој библиотеци постоји Gif Browser који анонимизује претраге и слике које пружа, треба само да га користите.

### Перформансе

Молимо отворите тикет са примером за репродукцију, укључујући уређај који сте користили, ако уочите било какав проблем са перформансама. Перформансе имају висок приоритет
у свим FastComments библиотекама.
---