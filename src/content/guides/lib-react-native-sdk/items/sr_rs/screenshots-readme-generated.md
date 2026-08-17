Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Живо коментарисање</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Живо коментарисање, светла тема"/></td>
    <td align="center"><b>Тамна тема</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Живо коментарисање, тамна тема"/></td>
    <td align="center"><b>Живи ћаскање</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Претподешавање за живи ћаскање"/></td>
  </tr>
</table>

### Уређивач богате текста

Ова библиотека користи [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) за уређивање богате текста, што пружа моћно WYSIWYG искуство уређивања. Исти уређивач покреће iOS, Android и веб (преко `react-native-web`), тако да се уређивач понаша доследно на свим платформама са једном имплементацијом.

`react-native-enriched` захтева нову архитектуру React Native (Fabric) на native (подразумевано од RN 0.76, опција за укључење на RN 0.72-0.75), и bundler који резолује услове `exports` пакета. Овај SDK је развијан и тестиран против RN 0.81 / React 19. Исти уређивач такође ради на вебу преко `react-native-web`; веб издање enriched уређивача још увек је означено као експериментално у извору.

### Виџети

SDK испоручује три виџета, који одражавају FastComments Android SDK:

- `FastCommentsLiveCommenting` - коментарисање у теми са гласовима, одговорима, пагинацијом, помињањима, обавештењима и живим ажурирањима.
- `FastCommentsLiveChat` - претподешавање за ћаскање над истим мотором: хронолошке поруке са новим на дну, уређивач испод листе, живи заглавље (тачка везе + број корисника), бесконачна историја учитана скроловањем нагоре, аутоматско скроловање ка новим порукама, без гласова или нити одговора. Свако претподешавање може се преписати преко `config`.
- `FastCommentsFeed` - друштвени фид са уређивачем постова, медијима, реакцијама, праћењима и живим банерима за нове постове.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Теме

Подразумевани изглед се генерише из скупа семантичких дизајнерских токена (`FastCommentsTheme`): боје, размак, радијус, величине фонта, тежине фонта и величине аватара. Проследите парцијалне преписе токена (типа `FastCommentsThemeOverrides`) преко `theme` пропа на било ком виџету и цео стабло стила ће се доследно преобликовати:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Тамни режим је само један скуп токена удаљен:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` проп и даље прихвата сирово `IFastCommentsStyles` стабло за прецизну контролу. Када су `theme` и `styles` оба прослеђени, експлицитни стилови имају предност над тематским стаблом; када је прослеђен само `styles`, он у потпуности замењује подразумевано (оригинално понашање, тако да постојеће интеграције и теме нису утицане). `setupDarkModeSkin` је застарео у корист `theme` пропа.

### Опције конфигурације

Ова библиотека има за циљ да подржи све конфигурационе опције дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), као и веб имплементација.

Поред тога, React Native додаје неколико SDK-специфичних опција преко `FastCommentsRNConfig`:

- `hideTopBar` - сакрива траку пријављеног корисника / звона за обавештења приказану изнад уређивача.
- `usePressToEdit` - притисни и држи коментар да би отворио његов мени.
- `disableDownVoting` - сакрива дугмад за негативно гласање.
- `renderCommentInline` - приказује информације о коментатору унутар истог HTML блока као садржај коментара.
- `renderLikesToRight` - премешта област гласова/лајкова удесно од коментара уместо испод.
- `renderDateBelowComment` - приказује датум испод коментара.
- `showLiveStatus` - приказује траку заглавља у стилу ћаскања „Live“ + број корисника изнад коментара.
- `useInlineSubmitButton` - приказује дугме за слање као икону унутар уређивача.
- `countAboveToggle` - заједно са `useShowCommentsToggle`, колико коментара се приказује изнад прекидача „Show Comments“.
- `preserveFeedScrollPosition` - `FastCommentsFeed` памти свој скрол офсет током укидања/поновног монтирања (подразумевано true).

### FastComments концепти

Главни концепти које треба знати за почетак су `tenantId` и `urlId`. `tenantId` је ваш идентификатор налога на FastComments.com. `urlId` је где ће се везати теме коментара. Ово може бити URL странице, или ID производа, ID чланка, итд.

### Локализација

Сав текст који се приказује кориснику у овим виџетима (ознаке дугмади, placeholder-и, празна стања, релативни датуми као „пре 5 минута“, поруке о грешкама, итд.) је **серверски управљан**. Компоненте не садрже фиксне енглеске стрингове; они рендерују преводе које FastComments обезбеђује за захтевани локал.

Да бисте захтевали локал, поставите `locale` у вашој конфигурацији:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Када `locale` није постављен, FastComments служи подразумевани језик тенанта.

**Уређивање текста:** преводи се управљају у вашем FastComments контролном панелу, а не у овом SDK-у. Да промените формулацију, препишите подразумевани текст или додајте језик, уредите преводе за ваш налог у контролном панелу – измена се аутоматски прихвата у виџетима без потребе за новим издањем апликације. SDK не испоручује енглеске резерве, тако да сваки кључ који оставите празним у контролном панелу ће се приказати празно; држите кључеве попуњеним за сваки подржани локал.

### Обавештења корисника

FastComments подржава обавештења за [многе сценарије](https://docs.fastcomments.com/guide-notifications.html). Обавештења су конфигурисана, могу се искључити глобално или на нивоу обавештења/коментара, и подржавају претплате на нивоу странице тако да корисници могу да се претплате на теме одређене странице или чланка.

На пример, могуће је користити Secure SSO за аутентификацију корисника, а затим периодично проверити непрочитана обавештења и послати их кориснику.

Погледајте [пример AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за то како добити и превести непрочитана корисничка обавештења.

### Gif прегледач

Подразумевано, избор слика или gif-ова није омогућен. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за то како подржати отпремање слика и gif-ова. Постоји Gif прегледач који анонимизује претраге и слике доступне у овој библиотеци, једноставно га користите.

### Перформансе

Молимо отворите тикет са примером за репродукцију, укључујући уређај који се користи, ако уочите било какве проблеме са перформансама. Перформансе су приоритет у свим FastComments библиотекама.

---