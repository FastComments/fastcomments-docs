Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Commenting</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live commenting, light theme"/></td>
    <td align="center"><b>Dark Theme</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live commenting, dark theme"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Уређивач богате текста

Ова библиотека користи [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) за уређивање богате текста, који пружа моћно WYSIWYG искуство уређивања. Исти уређивач покреће iOS, Android и веб (преко `react-native-web`), тако да се композитор понаша конзистентно на свакој платформи са једном имплементацијом.

`react-native-enriched` захтева Нову архитектуру React Native (Fabric) на изворном (подразумевано од RN 0.76, опција на RN 0.72-0.75), и пакет менаџер који резолује услове `exports` пакета. Ова SDK је развијана и тестирана са RN 0.81 / React 19. Исти уређивач такође ради на вебу преко `react-native-web`; веб изградња enriched уређивача још увек је означена као експериментална.

### Виџети

SDK испоручује три виџета, који огледају FastComments Android SDK:

- `FastCommentsLiveCommenting` - коментарисање у нитима са гласовима, одговорима, пагинацијом, споменима, обавештењима и живим ажурирањима.
- `FastCommentsLiveChat` - пресет за ћаскање над истим мотором: хронолошке поруке са новим на дну, композитор испод листе, живи насловани трак (тачка конекције + број корисника), бесконачна историја учитана превлачењем нагоре, аутоматско скроловање ка новим порукама, без гласова или нити одговора. Сваки пресет може се преписати преко `config`.
- `FastCommentsFeed` - друштвени ток са композитором за објаве, медијима, реакцијама, праћењима и живим банерима за нове објаве.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Теме

Подразумевани изглед се генерише из скупа семантичких токена за дизајн (`FastCommentsTheme`): боје, размак, радијус, величине фонта, дебљине фонта и величине аватара. Проследите парцијалне преписе токена (типа `FastCommentsThemeOverrides`) преко `theme` пропа на било ком виџету и цело стилизово стабло ће се доследно поново стиловати:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Тамни режим је само један скуп токена даље:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` проп и даље прихвата сирово `IFastCommentsStyles` стабло за прецизну контролу. Када се `theme` и `styles` оба доставе, експлицитни стилови имају предност над тематским стаблом; када је достављен само `styles`, он у потпуности замењује подразумевано (оригинално понашање, тако да постојеће интеграције и теме нису утицане). `setupDarkModeSkin` је застарео у корист `theme` пропа.

### Опције конфигурације

Ова библиотека има за циљ подржати све опције конфигурације дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), управо као веб имплементација.

Поред тога, React Native додаје неколико SDK-специфичних опција преко `FastCommentsRNConfig`:

- `hideTopBar` - сакрија траку пријављеног корисника / звона за обавештења који се приказује изнад композитора.
- `usePressToEdit` - притисни и задржи коментар за отварање његовог менија.
- `disableDownVoting` - сакрива дугмад за негативно гласање.
- `renderCommentInline` - приказује информације о коментатору унутар истог HTML блока као садржај коментара.
- `renderLikesToRight` - премешта област гласа/лајка у десну страну коментара уместо испод.
- `renderDateBelowComment` - приказује датум испод коментара.
- `showLiveStatus` - приказује насловану траку у стилу ћаскања „Live“ + број корисника изнад коментара.
- `useInlineSubmitButton` - приказује дугме за слање као икону унутар композитора.
- `countAboveToggle` - у комбинацији са `useShowCommentsToggle`, колико коментара се приказује изнад прекидача „Show Comments“.
- `preserveFeedScrollPosition` - `FastCommentsFeed` памти свој прокручени офсет приликом одмонтирања/монтирања (подразумевано true).

### Концепти FastComments

Главни концепти које треба разумети за почетак су `tenantId` и `urlId`. `tenantId` је идентификатор вашег FastComments.com налога. `urlId` је на шта се вежу коментарске нитове. То може бити URL странице, идентификатор производа, идентификатор чланка, итд.

### Локализација

Сав текст који се приказује кориснику у овим виџетима (ознаке дугмади, placeholder-и, празна стања, релативни датуми као „пре 5 минута“, поруке о грешкама, итд.) је **диригирано са сервера**. Компоненте не кодирамо фиксне енглеске ниске; они приказују преводе које FastComments доставља за тражени локал.

За захтев за локал, поставите `locale` у вашој конфигурацији:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Када `locale` није постављен, FastComments користи подразумевани језик станара.

**Уређивање текста:** преводи се управљају у вашем FastComments контролном панелу, не у овом SDK‑у. За промену формулисања, препишете подразумевани копи, или додајте језик, уредите преводе за ваш налог у контролном панелу – промена се аутоматски прихвата у виџетима без потребе за новим издањем апликације. SDK не испоручује енглеске резерве, тако да било који кључ који оставите празним у контролном панелу приказује празно; држите кључеве попуњеним за сваки подржани локал.

### Обавештења корисника

FastComments подржава обавештења за [много сценарија](https://docs.fastcomments.com/guide-notifications.html). Обавештења су конфигурисана, могу се глобално искључити или на нивоу обавештења/коментара, и подржавају претплате на ниво странице тако да корисници могу да се претплате на нишке одреđене странице или чланка.

На пример, могуће је користити Secure SSO за аутентификацију корисника, а затим периодично испитивати непрочитана обавештења и послати их кориснику.

Погледајте [пример AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за то како добити и превести непрочитана обавештења корисника.

### Gif Прегледач

Подразумевано, нема одабира слика или gif‑а. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за то како подржати отпремање слика и gif‑а. Постоји Gif Прегледач који анонимизује претраге и слике које се пружају у овој библиотеци, требало би само да га користите.

### Перформансе

Молимо отворите тикет са примером за понављање, укључујући уређај који се користи, ако идентификујете било какве проблеме са перформансама. Перформансе су приоритетни аспект свих FastComments библиотека.