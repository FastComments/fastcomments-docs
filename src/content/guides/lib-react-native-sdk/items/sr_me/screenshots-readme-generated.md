Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Живо коментарисање</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Живо коментарисање, светла тема"/></td>
    <td align="center"><b>Тамна тема</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Живо коментарисање, тамна тема"/></td>
    <td align="center"><b>Живи чет</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Подешавање живог чета"/></td>
  </tr>
</table>

### Уређивач обогаћеног текста

Ова библиотека користи [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) за уређивање обогаћеног текста, који пружа снажно WYSIWYG искуство уређивања. Исти уређивач покреће iOS, Android и веб (кроз `react-native-web`), тако да се композиитор понаша доследно на свим платформама уз једну имплементацију.

`react-native-enriched` захтева React Native New Architecture (Fabric) на нативној страни (подразумевано од RN 0.76, опционално на RN 0.72-0.75), и бандлер који разрешава package `exports` услове. Овај SDK је развијан и тестиран против RN 0.81 / React 19. Исти уређивач такође ради на вебу преко `react-native-web`; web build обогаћеног уређивача је и даље означен као експериментални у upstream-у.

### Видгети

SDK садржи три видгета, по узору на FastComments Android SDK:

- `FastCommentsLiveCommenting` - нитовано коментарисање са гласањем, одговорима, пагинацијом, поменима, нотификацијама и живим ажурирањима.
- `FastCommentsLiveChat` - пресет ћаскања над истим енџином: хронолошке поруке са новим порукама на дну, композиитор испод листе, жива заглавна трака (тачка везе + број корисника), бесконачна историја која се учитава скроловањем нагоре, аутоскрол на нове поруке, без гласања или нитова одговора. Сваки пресет може се променити преко `config`.
- `FastCommentsFeed` - социјални фид са композиитором поста, медијима, реакцијама, праћењем и банерима за нове постове у реалном времену.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Теме

Подразумевани изглед се генерише из скупа семантичких дизајн токена (`FastCommentsTheme`): боје, размак, радијуси, величине фонтова, тежине фонта и величине аватара. Проследите делимична преклапања токена (типизовано као `FastCommentsThemeOverrides`) преко проп-а `theme` на било ком видгету и цела стабло стилова ће се доследно редизајнирати:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Тамни режим се добија уз један скуп токена:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Проп `styles` и даље прихвата сирово дрво `IFastCommentsStyles` за прецизну контролу. Када су и `theme` и `styles` оба присутни, експлицитни стилови имају предност над тематизованим дрветом; када је само `styles` прослеђен, он у потпуности замењује подразумеване стилове (првобитно понашање, тако да постојеће интеграције и skin-ови остају нетакнути). `setupDarkModeSkin` је застарео у корист проп-а `theme`.

### Опције конфигурације

Ова библиотека има за циљ да подржи све опције конфигурације дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), баш као веб имплементација.

Поред тога, React Native додаје неке опције специфичне за SDK преко `FastCommentsRNConfig`:

- `hideTopBar` - сакрива траку са пријављеним корисником / звоном за нотификације која се приказује изнад композитора.
- `usePressToEdit` - притисните и држите коментар да бисте отворили његов мени.
- `disableDownVoting` - сакрива дугмад за негативно гласање.
- `renderCommentInline` - рендерује информације о коментатору унутар истог HTML блока као садржај коментара.
- `renderLikesToRight` - помера област гласања/лајкова на десну страну коментара уместо испод њега.
- `renderDateBelowComment` - рендерује датум испод коментара.
- `showLiveStatus` - приказује заглавну траку у стилу чета "Live" + број корисника изнад коментара.
- `useInlineSubmitButton` - рендерује дугме за слање као икону унутар композитора.
- `countAboveToggle` - уз `useShowCommentsToggle`, колико коментара се приказује изнад тастера "Show Comments".
- `preserveFeedScrollPosition` - `FastCommentsFeed` памти свој офсет скроловања преко одмонтирања/поновног монтирања (подразумевано true).

### FastComments концепти

Главни концепти које треба познавати да бисте почели су `tenantId` и `urlId`. `tenantId` је идентификатор вашег FastComments.com налога. `urlId` је на шта ће нитови коментара бити везани. То може бити URL странице, или ид производа, ид чланка итд.

### Нотификације корисника

FastComments подржава нотификације за [многе сценарије](https://docs.fastcomments.com/guide-notifications.html). Нотификације су конфигурабилне,
корисници се могу искључити глобално или по нотификацији/коментару, и подржавају претплате на нивоу странице тако да се корисници могу претплатити на нити одређене странице или чланка.

На пример, могуће је користити Secure SSO за аутентификацију корисника и онда периодично чекати непрочитане нотификације и пуш-овати их кориснику.

Погледајте [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за то како добити и преводити непрочитане нотификације корисника.

### Gif прегледач

Подразумевано, није дозвољен избор слика или gif-ова. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за то како
подржати отпремање слика и gif-ова. У овој библиотеци постоји Gif прегледач који анонимизује претраге и слике које се достављају, само га је потребно користити.

### Перформансе

Молимо отворите тикет са примером за репродукцију, укључујући уређај који сте користили, ако идентификујете било које проблеме са перформансама. Перформансе су приоритет у свим FastComments библиотекама.