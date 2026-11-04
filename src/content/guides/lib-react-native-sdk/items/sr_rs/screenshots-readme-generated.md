Коментисање уживо са нитима, са аватарима, угнежђеним одговорима, гласовима и уграђеним rich-text композером, уз тамну тему и preset за live-chat (приказано овде преко `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Коментисање уживо</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Коментисање уживо, светла тема"/></td>
    <td align="center"><b>Тамна тема</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Коментисање уживо, тамна тема"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Rich Text уређивач

Ова библиотека користи [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) за rich text уређивање, који пружа моћно WYSIWYG искуство уређивања. Исти уређивач покреће iOS, Android и веб (преко `react-native-web`), тако да се композер понаша доследно на свакој платформи са једном имплементацијом.

`react-native-enriched` захтева React Native New Architecture (Fabric) на нативним платформама (подразумевано од RN 0.76, опционо на RN 0.72-0.75), и bundler који подржава `exports` услове пакета. Овај SDK је развијен и тестиран са RN 0.81 / React 19. Исти уређивач ради и на вебу преко `react-native-web`; веб верзија enriched уређивача је и даље означена као експериментална у upstream пројекту.

### Видгети

SDK испоручује три видгета, у складу са FastComments Android SDK:

- `FastCommentsLiveCommenting` - коментисање у нитима са гласовима, одговорима, пагинацијом, помињањима, нотификацијама и ажурирањима уживо.
- `FastCommentsLiveChat` - chat preset над истим погоном: хронолошке поруке са новим на дну, композер испод листе, live header трака (тачка конекције + број корисника), бесконачна историја која се учитава скроловањем нагоре, аутоматски скрол до нових порука, без гласова и одговора у нитима. Сваки preset може бити преписан преко `config`.
- `FastCommentsFeed` - друштвени фид са композером за објаве, медијима, реакцијама, праћењем и live банерима за нове објаве.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Теме

Подразумевани изглед се генерише из скупа семантичких design токена (`FastCommentsTheme`): боје, размаци, радијуси, величине фонта, дебљине фонта и величине аватара. Проследите делимичне преписке токена (типизирано као `FastCommentsThemeOverrides`) кроз `theme` проп на било ком видгету и цео стилски дрво ће се доследно рестилизовати:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Тамни режим је удаљен само један скуп токена:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` проп и даље прихвата сирово `IFastCommentsStyles` дрво за прецизну контролу. Када су `theme` и `styles` оба задата, експлицитни стилови имају предност над темом; када је задат само `styles`, он потпуно замењује подразумеване вредности (оригинално понашање, тако да постојеће интеграције и скинови нису погођени). `setupDarkModeSkin` је застарео у корист `theme` пропа.

### Опције конфигурације

Ова библиотека има за циљ да подржи све опције конфигурације дефинисане у [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), баш као веб имплементација.

Поред тога, React Native додаје неколико опција специфичних за SDK преко `FastCommentsRNConfig`:

- `hideTopBar` - сакриј траку са пријављеним корисником / звоном за нотификације изнад композера.
- `usePressToEdit` - притисните и задржите коментар да бисте отворили његов мени.
- `disableDownVoting` - сакриј дугмад за гласање надоле.
- `renderCommentInline` - прикажи информације о коментатору унутар истог HTML блока као садржај коментара.
- `renderLikesToRight` - премести област гласања/лајкова десно од коментара уместо испод њега.
- `renderDateBelowComment` - прикажи датум испод коментара.
- `showLiveStatus` - прикажи chat-style "Live" траку са бројем корисника изнад коментара.
- `useInlineSubmitButton` - прикажи дугме за слање као икону унутар композера.
- `countAboveToggle` - са `useShowCommentsToggle`, колико коментара се приказује изнад тумбача "Show Comments".
- `preserveFeedScrollPosition` - `FastCommentsFeed` памти своју скрол позицију између unmount/remount (подразумевано true).

### FastComments концепти

Главни концепти које треба да знате за почетак су `tenantId` и `urlId`. `tenantId` је идентификатор вашег FastComments.com налога. `urlId` је оно на шта ће бити везане нити коментара. То може бити URL странице, или id производа, id чланка, итд.

### Локализација

Сав текст окренут кориснику у овим видгетима (ознаке дугмади, placeholder-и, празна стања, релативни датуми попут "пре 5 минута", поруке о грешкама, итд.) је **управљан са сервера**. Компоненте не
hard-code-ују енглеске стрингове; оне приказују преводе које FastComments испоручује за захтевани locale.

Да бисте захтевали locale, поставите `locale` у својој конфигурацији:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Када `locale` није постављен, FastComments испоручује подразумевани језик tenant-а.

**Уређивање текста:** преводи се управљају у вашем FastComments dashboard-у, а не у овом SDK-у.
Да бисте променили формулацију, преписали подразумевани текст или додали језик, уредите преводе за свој
налог у dashboard-у - промена се аутоматски преузима у видгетима без потребе за
објављивањем апликације. SDK не испоручује енглеске fallback вредности, тако да сваки кључ који испразните у dashboard-у
приказује празно; држите кључеве попуњеним за сваки locale који подржавате.

### Корисничке нотификације

FastComments подржава нотификације за [многе сценарије](https://docs.fastcomments.com/guide-notifications.html). Нотификације су конфигурабилне,
могу се глобално или на нивоу нотификације/коментара искључити, и подржавају претплате на нивоу странице тако да корисници могу да се претплате на нити
конкретне странице или чланка.

На пример, могуће је користити Secure SSO за аутентификацију корисника, а затим периодично проверавати непрочитане нотификације и слати их кориснику.

Погледајте [пример AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за то како да преузмете и прикажете непрочитане корисничке нотификације.

### Gif прегледач

Подразумевано, избор слика или gif-ова није омогућен. Погледајте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за то како
да подржите отпремање слика и gif-ова. Постоји Gif Browser који анонимизује претраге и слике обезбеђене у овој библиотеци, само га треба користити.

### Перформансе

Ако приметите било какве проблеме са перформансама, отворите тикет са примером за репродукцију, укључујући и коришћени уређај. Перформансе су прворазредни приоритет
сви FastComments библиотека.