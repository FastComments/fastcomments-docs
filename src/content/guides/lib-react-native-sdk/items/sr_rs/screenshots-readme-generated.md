#### Скин: Erebus
![Скин: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Скин: Подразумевани
![Скин: Подразумевани](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Нативни WYSIWYG уређивач са подршком за слике!
![Нативни WYSIWYG уређивач са подршком за слике](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Уређивач богатог текста

Ова библиотека користи [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) за уређивање богатог текста, што пружа моћно WYSIWYG искуство уређивања. Исти уређивач покреће iOS, Android и web (путем `react-native-web`), тако да се састављач понаша доследно на свакој платформи са једном имплементацијом.

`react-native-enriched` захтева React Native New Architecture (Fabric) на нативном нивоу, и бандлер који решава услове `exports` у пакету (Metro са package exports / RN 0.72+). Подршка за web је тренутно експериментална.

### Опције конфигурације

Ова библиотека има за циљ да подржи све опције конфигурације дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), као и веб имплементација.

### Основни концепти FastComments

Главни концепти које треба познавати да бисте почели су `tenantId` и `urlId`. `tenantId` је идентификатор вашег FastComments.com налога. `urlId` је за шта ће бити везане нити коментара. То може бити URL странице, или id производа, id чланка итд.

### Корисничке обавештења

FastComments подржава нотификације за [многе сценарије](https://docs.fastcomments.com/guide-notifications.html). Нотификације су конфигурисане, могу се искључити глобално или на нивоу нотификације/коментара, и подржавају претплате на нивоу странице тако да се корисници могу претплатити на нити одређене странице или чланка.

На пример, могуће је користити Secure SSO за аутентификацију корисника и затим повремено проверавати непрочитана обавештења и слати их кориснику.

Погледајте [пример AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за то како добити и трансформисати непрочитана корисничка обавештења.

### GIF претраживач

По подразумеваној вредности, избор слика или GIF-ова није омогућен. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за то како подржати отпремање слика и GIF-ова. У овој библиотеци постоји GIF претраживач који анонимизује претраге и слике које обезбеђује библиотека — потребно је само да га користите.

### Перформансе

Молимо отворите тикет са примером за репродукцију, укључујући који уређај је коришћен, ако уочите било какве проблеме са перформансама. Перформансе су приоритет у свим FastComments библиотекама.