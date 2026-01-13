#### Тема: Erebus
![Тема: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Тема: Подразумевана
![Тема: Подразумевана](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Нативни WYSIWYG уредник са подршком за слике!
![Нативни WYSIWYG уредник са подршком за слике](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Уређивач обогаћеног текста

Ова библиотека користи 10tap editor за функционалност уређивања обогаћеног текста, што пружа моћно WYSIWYG искуство уређивања.

### Опције конфигурације

Ова библиотека има за циљ да подржи све опције конфигурације дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), као и веб имплементација.

### Концепти FastComments

Главни концепти које треба знати да бисте почели су `tenantId` и `urlId`. `tenantId` је идентификатор вашег FastComments.com налога. `urlId` је место на које ће бити везане нити коментара. Ово може бити URL странице, или идентификатор производа, идентификатор чланка итд.

### Обавештења корисника

FastComments подржава обавештења за [много сценарија](https://docs.fastcomments.com/guide-notifications.html). Обавештења су конфигурисана, корисник може отказати претплату глобално или на нивоу обавештења/коментара, и подржава претплате на нивоу странице тако да се корисници могу претплатити на нити одређене странице или чланка.

На пример, могуће је користити Secure SSO за аутентификацију корисника, а затим периодично проверавати непрочитана обавештења и слати их кориснику.

Погледајте [пример AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за то како добити и превести непрочитана обавештења корисника.

### GIF прегледач

По подразумеваној вредности није омогућен избор слика или GIF-ова. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за то како подржати отпремање слика и GIF-ова. Постоји GIF прегледач који анонимизује претраге и слике које пружа ова библиотека, само га употребите.

### Перформансе

Ако откријете било какве проблеме са перформансама, отворите тикет са примером за репродукцију, укључујући и коришћени уређај. Перформансе су приоритет у свим FastComments библиотекама.