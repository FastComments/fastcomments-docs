Уживо нитовано коментарисање са аватарима, унутрашњим одговорима, гласањима и уграђеним уређивачем форматираног текста, плус тамна тема и претсет за уживо ћаскање (приказано овде рендеровано преко `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Уживо коментарисање</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Уживо коментарисање, светла тема"/></td>
    <td align="center"><b>Тамна тема</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Уживо коментарисање, тамна тема"/></td>
    <td align="center"><b>Уживо ћаскање</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Претсет за уживо ћаскање"/></td>
  </tr>
</table>

### Уређивач форматираног текста

Ова библиотека користи [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) за уређивање форматираног текста, што обезбеђује моћно WYSIWYG искуство уређивања. Исти уређивач покреће iOS, Android и web (путем `react-native-web`), тако да се композер понаша доследно на свакој платформи уз једну имплементацију.

`react-native-enriched` захтева React Native New Architecture (Fabric) на нативним платформама (подразумевано од RN 0.76, изборно у RN 0.72-0.75), и bundler који решава package `exports` услове. Овај SDK је развијен и тестиран против RN 0.81 / React 19. Исти уређивач такође ради на web кроз `react-native-web`; веб build enriched уређивача још увек је означен као експерименталан upstream.

### Видгети

SDK испоруучује три видгета, по узору на FastComments Android SDK:

- `FastCommentsLiveCommenting` - нитовано коментарисање са гласањима, одговорима, пагинацијом, помињањима, обавештењима и уживо ажурирањима.
- `FastCommentsLiveChat` - претсет ћаскања на истом енджину: хронолошке поруке са новим порукама на дну, композер испод листе, уживо хедер трака (тачка везе + број корисника), бесконачна историја која се учитава клизањем нагоре, аутоматско скроловање на нове поруке, без гласања или нитова одговора. Сваки претсет може бити преовладан преко `config`.
- `FastCommentsFeed` - друштвени фид са композером за постове, медијима, реакцијама, праћењима и уживо банерима за нове постове.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Тематизација

Подразумевани изглед се генерише из скупа семантичких дизајн токена (`FastCommentsTheme`): боја, размaка, радијуса, величина фонта, тежина фонта и величина аватара. Проследите делимична преклапања токена (типизирана као `FastCommentsThemeOverrides`) кроз `theme` проп на било ком видгету и цело стабло стилова ће се доследно рестиловати:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

За тамни режим потребан је само један сет токена:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` проп и даље прихвата сироки `IFastCommentsStyles` tree за прецизну контролу. Када су и `theme` и `styles` прослеђени, експлицитни стилови имају предност над тематским стаблом; када се проследи само `styles`, он у потпуности замењује подразумеване (оригинално понашање, тако да постојеће интеграције и скинови нису погођени). `setupDarkModeSkin` је застарео у корист `theme` проп-а.

### Опције конфигурације

Ова библиотека тежи да подржи све опције конфигурације дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), баш као и web имплементација.

Поред њих, React Native додаје неколико SDK-специфичних опција преко `FastCommentsRNConfig`:

- `hideTopBar` - сакриј траку са пријављеним корисником / звона обавештења која се приказује изнад композера.
- `usePressToEdit` - притисните и држите коментар да бисте отворили његов мени.
- `disableDownVoting` - сакриј дугмад за негативно гласање.
- `renderCommentInline` - прикажи информације о аутору коментара унутар истог HTML блока као садржај коментара.
- `renderLikesToRight` - премести област за гласање/лајкове на десну страну коментара уместо испод њега.
- `renderDateBelowComment` - прикажи датум испод коментара.
- `showLiveStatus` - прикажи чат-стил "Live" + број корисника у хедер траци изнад коментара.
- `useInlineSubmitButton` - прикажи дугме за слање као икону унутар композера.
- `countAboveToggle` - уз `useShowCommentsToggle`, колико коментара се прикаже изнад прекидача "Show Comments".
- `preserveFeedScrollPosition` - `FastCommentsFeed` памти свој scroll offset преко unmount/remount (подразумевано true).

### Концепти FastComments

Главни концепти које треба да знате да бисте почели су `tenantId` и `urlId`. `tenantId` је идентификатор вашег FastComments.com налога. `urlId` је за шта ће нитови коментара бити везани. То може бити URL странице, id производа, id чланка, итд.

### Корисничка обавештења

FastComments подржава обавештења за [многе сценарије](https://docs.fastcomments.com/guide-notifications.html). Обавештења су конфигурисана, могу се отказати глобално или на нивоу обавештења/коментара, и подржавају претплате на нивоу странице тако да се корисници могу претплатити на нитове одређене странице или чланка.

На пример, могуће је користити Secure SSO за аутентификацију корисника и затим периодично проверавати непрочитана обавештења и проследити их кориснику.

Погледајте [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за то како да добијете и преведете непрочитана корисничка обавештења.

### Gif претраживач

По подразумеваној вредности није омогућен избор слика или гифова. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за то како да подржите отпремање слика и гифова. У овој библиотеци постоји Gif Browser који анонимизује претраге и слике које се достављају, потребно је само да га користите.

### Перформансе

Молимо отворите тикет са примером за репродуковање, укључујући уређај који сте користили, ако идентификујете било какве проблеме са перформансама. Перформансе су приоритет у свим FastComments библиотекама.