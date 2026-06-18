Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Живо коментарисање</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Живо коментарисање, свјетла тема"/></td>
    <td align="center"><b>Тамна тема</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Живо коментарисање, тамна тема"/></td>
    <td align="center"><b>Живи чет</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Подешавање живог чета"/></td>
  </tr>
</table>

### Уређивач богатог текста

Ова библиотека користи [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) за уређивање богатог текста, што пружа снажно WYSIWYG искуство уређивања. Исти уређивач покреће iOS, Android и веб (преко `react-native-web`), тако да се композитор понаша досљедно на свакој платформи са једном имплементацијом.

`react-native-enriched` захтијева React Native New Architecture (Fabric) на нативном слоју (подразумјевано од RN 0.76, опциони на RN 0.72-0.75), и бандлер који решава `exports` услове пакета. Овај SDK је развијен и тестиран против RN 0.81 / React 19. Исти уређивач такође ради на вебу кроз `react-native-web`; веб build enriched уређивача је још увијек означен као експерименталан у upstream-у.

### Видгети

SDK испоручује три видгета, који одражавају FastComments Android SDK:

- `FastCommentsLiveCommenting` - темско (threaded) коментарисање са гласањима, одговорима, пагинацијом, помињањима, обавијестима и уживо ажурирањима.
- `FastCommentsLiveChat` - пресет четa користећи исти мотор: хронолошке поруке са новим порукама на дну, композитор испод листе, жива трака заглавља (тачка везе + број корисника), неограничена историја учитавана клизањем горе, аутоматско скроловање до нових порука, без гласања или темског одговарања. Сваки пресет се може преписати преко `config`.
- `FastCommentsFeed` - друштвени фид са композитором постова, медијима, реакцијама, праћењима и уживо банерима за нове постове.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Теме

Подразумјевани изглед се генерише из скупа семантичких дизајн токена (`FastCommentsTheme`): боје, размак, радијус, величине фонтова, тежине фонтова и величине аватара. Прослиједите дијеловне замјене токена (типизирано `FastCommentsThemeOverrides`) кроз `theme` проп на било ком видгету и цијело стабло стилова ће се досљедно рестиловати:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Тамни режим се добија са само једним сетом токена:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` проп и даље прихвата сирово `IFastCommentsStyles` стабло за прецизну контролу. Када су `theme` и `styles` оба присутни, експлицитни стилови имају предност над темираним стаблом; када је само `styles` присутан, он потпуно замјењује подразумјеване (оригинално понашање, тако да постојеће интеграције и скинови остају непогођени). `setupDarkModeSkin` је застарјело у корист `theme` пропа.

### Опције конфигурације

Ова библиотека циљано подржава све опције конфигурације дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), баш као и веб имплементација.

Поред тога, React Native додаје неколико опција специфичних за SDK преко `FastCommentsRNConfig`:

- `hideTopBar` - сакрије траку са пријављеним корисником / звоницом за обавијести која је видљива изнад композитора.
- `usePressToEdit` - притиском и држањем коментара отвори његов мени.
- `disableDownVoting` - сакрије дугмад за негативно гласање.
- `renderCommentInline` - прикаже податке о коментатору унутар истог HTML блока као садржај коментара.
- `renderLikesToRight` - премјести подручје гласања/лајкова на десну страну коментара уместо испод њега.
- `renderDateBelowComment` - прикаже датум испод коментара.
- `showLiveStatus` - прикаже чет-стил "Live" + број корисника у заглављу изнад коментара.
- `useInlineSubmitButton` - прикаже дугме за слање као иконицу унутар композитора.
- `countAboveToggle` - уз `useShowCommentsToggle`, колико коментара ће се приказати изнад тастера "Показати коментаре".
- `preserveFeedScrollPosition` - `FastCommentsFeed` памти свој offset скрола преко unmount/remount-а (подразумјевано true).

### FastComments концепти

Главни концепти које треба познавати да бисте почели су `tenantId` и `urlId`. `tenantId` је идентификатор вашег FastComments.com налога. `urlId` је везан за гдје ће ф нитне теме коментара бити везане. Ово може бити URL странице, или ид производа, ид чланка, итд.

### Обавештења корисника

FastComments подржава обавештења за [много сценарија](https://docs.fastcomments.com/guide-notifications.html). Обавештења су конфигурисана,
могу се искључити глобално или на нивоу обавештења/коментара, и подржавају претплате на нивоу странице тако да се корисници могу претплатити на теме одређене
странице или чланка.

На примјер, могуће је користити Secure SSO да аутентификујете корисника а затим периодично провјеравати непрочитана обавештења и пушовати их кориснику.

Погледајте [пример AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за то како добити и превести непрочитана корисничка обавештења.

### GIF претраживач

По подразумјевaњу, није омогућен избор слика или GIF-ова. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за то како
подржати отпремање слика и GIF-ова. У овој библиотеци постоји GIF претраживач који анонимизује претраге и слике које пружа, само га треба користити.

### Перформансе

Молимо отворите тикет са примером за репродукцију, укључујући уређај који сте користили, ако примјетите било какве проблеме са перформансама. Перформансе су приоритет
у свим FastComments библиотекама.