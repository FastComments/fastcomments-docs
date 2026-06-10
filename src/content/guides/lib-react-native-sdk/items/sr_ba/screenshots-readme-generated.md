#### Тема: Erebus
![Тема: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Тема: Default
![Тема: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Нативни WYSIWYG уређивач са подршком за слике!
![Нативни WYSIWYG уређивач са подршком за слике](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Уређивач богатог текста

Ова библиотека користи [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) за уређивање богатог текста, што пружа снажно WYSIWYG искуство уређивања. Исти уређивач покреће iOS, Android и веб (путем `react-native-web`), па се уређивач понаша доследно на свим платформама са једном имплементацијом.

`react-native-enriched` захтијева React Native New Architecture (Fabric) на нативној страни, као и пакер који решава услове `exports` пакета (Metro са package exports / RN 0.72+). Подршка за веб је тренутно експериментална.

### Опције конфигурације

Ова библиотека има за циљ да подржи све опције конфигурације дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), баш као и веб имплементација.

### Основни појмови FastComments

Главни појмови којима треба обратити пажњу за почетак су `tenantId` и `urlId`. `tenantId` је идентификатор вашег FastComments.com налога. `urlId` је где ће нити коментара бити повезане. То може бити URL странице, или ид производа, или ид чланка итд.

### Обавештења корисника

FastComments подржава нотификације за [много сценарија](https://docs.fastcomments.com/guide-notifications.html). Нотификације су конфигурисане, могуће је отказати их глобално или на нивоу појединачне нотификације/коментара, и подржавају претплате на нивоу странице тако да се корисници могу пријавити за нити одређене странице или чланка.

На пример, могуће је користити Secure SSO за аутентификацију корисника и затим периодично провјеравати непрочитане нотификације и гурати их кориснику.

Погледајте [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за начин како добити и превести непрочитана корисничка обавештења.

### GIF прегледач

По подразумеваној поставци, избор слика или GIF-ова није омогућен. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за начин како подржати отпремање слика и GIF-ова. У овој библиотеци постоји GIF прегледач који анонимизује претраге и слике које обезбјеђује библиотека, само га морате користити.

### Перформансе

Ако идентификујете било какве проблеме са перформансама, отворите тикет са примером за репродукцију, укључујући уређај који је кориштен. Перформансе су приоритет свих FastComments библиотека.