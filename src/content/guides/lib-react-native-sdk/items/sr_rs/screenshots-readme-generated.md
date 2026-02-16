#### Тема: Erebus
![Тема: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Тема: Подразумевана
![Тема: Подразумевана](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Нативни WYSIWYG уређивач са подршком за слике!
![Нативни WYSIWYG уређивач са подршком за слике](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Уређивач богатог текста

Ова библиотека користи уређивач 10tap за уређивање богатог текста, што пружа моћно WYSIWYG искуство уређивања.

### Опције конфигурације

Ова библиотека има за циљ да подржи све опције конфигурације дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), као и веб имплементација.

### Концепти FastComments

Главни концепти које треба познавати да бисте почели су `tenantId` и `urlId`. `tenantId` је идентификатор вашег FastComments.com налога. `urlId` је место везивања нити коментара. Ово може бити URL странице, или идентификатор производа, идентификатор чланка, итд.

### Корисничке обавештења

FastComments подржава обавештења за [многе сценарије](https://docs.fastcomments.com/guide-notifications.html). Обавештења су конфигурисива, корисници могу да се одјаве глобално или на нивоу обавештења/коментара, и подржава претплате на нивоу странице тако да се корисници могу претплатити на нити одређене странице или чланка.

На пример, могуће је користити Secure SSO за аутентификацију корисника, а затим периодично проверавати непрочитана обавештења и прослеђивати их кориснику.

Погледајте [пример AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за то како добити и превести непрочитана корисничка обавештења.

### GIF претраживач

По подразумевaњу, селекција слика или GIF-ова није омогућена. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за то како подржати отпремање слика и GIF-ова. У овој библиотеци постоји GIF претраживач који анонимизује претраге и слике које се обезбеђују — све што треба да урадите је да га користите.

### Перформансе

Ако уочите било какве проблеме са перформансама, отворите тикет са примером за репродукцију, укључујући уређај који је коришћен. Перформансе су приоритет у свим FastComments библиотекама.