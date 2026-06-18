Коментари в реално време с нишки, аватари, вградения композер за форматиран текст, вложени отговори, гласувания, плюс тъмен режим и предварителна настройка за чат в реално време (показано тук чрез `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Коментари в реално време</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Коментари в реално време, светла тема"/></td>
    <td align="center"><b>Тъмен режим</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Коментари в реално време, тъмна тема"/></td>
    <td align="center"><b>Чат в реално време</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Предварителна настройка за чат в реално време"/></td>
  </tr>
</table>

### Rich Text Editor

Тази библиотека използва [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) за редактиране на форматиран текст, което осигурява мощно WYSIWYG изживяване при редактиране. Същият редактор захранва iOS, Android и уеб (чрез `react-native-web`), така че композерът се държи последователно на всички платформи с една реализация.

`react-native-enriched` изисква React Native New Architecture (Fabric) на native платформи (по подразбиране от RN 0.76, опция при RN 0.72-0.75), и bundler, който разрешава условието за `exports` на пакета. Това SDK е разработено и тествано спрямо RN 0.81 / React 19. Същият редактор също работи в уеб чрез `react-native-web`; уеб билдът на enriched редактора все още е маркиран като експериментален upstream.

### Widgets

SDK-то доставя три widgets, огледално на FastComments Android SDK:

- `FastCommentsLiveCommenting` - нишкови коментари с гласувания, отговори, пагинация, споменавания, известия и живи обновявания.
- `FastCommentsLiveChat` - настройка за чат върху същия двигател: хронологични съобщения с новите в долната част, композерът под списъка, жива горна лента (индикатор за връзка + брой потребители), неограничена история, зареждана при превъртане нагоре, автоматично прелистване до нови съобщения, без гласувания или нишкова структура от отговори. Всяка предварителна настройка може да бъде презаписана чрез `config`.
- `FastCommentsFeed` - социален фийд с композер за постове, медия, реакции, следвания и банери за нови постове в реално време.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

По подразбиране визията се генерира от набор от семантични дизайн токени (`FastCommentsTheme`): цветове, разстояния, радиуси, размери на шрифтове, тегла на шрифтове и размери на аватарите. Подайте частични презаписи на токените (типа `FastCommentsThemeOverrides`) чрез prop-а `theme` на всеки widget и цялото дърво със стилове ще се пренареди последователно:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Тъмният режим се постига с един набор токени:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Prop-ът `styles` все още приема сурово дърво `IFastCommentsStyles` за хирургичен контрол. Когато `theme` и `styles` са предоставени и двете, експлицитните стилове имат предимство пред тематизираното дърво; когато е предоставен само `styles`, той напълно замества подразбиращите се стойности (оригиналното поведение, така че настоящите интеграции и скинове не се засягат). `setupDarkModeSkin` е остаряло и се предпочита prop-ът `theme`.

### Configuration Options

Тази библиотека цели да поддържа всички опции за конфигуриране, дефинирани в [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), както при уеб реализацията.

Освен тях, React Native добавя няколко специфични за SDK опции чрез `FastCommentsRNConfig`:

- `hideTopBar` - скрива лентата с влезлия потребител / икона за известия, показвана над композера.
- `usePressToEdit` - натисни и задръж коментар, за да отвориш неговото меню.
- `disableDownVoting` - скрива бутоните за гласуване надолу.
- `renderCommentInline` - рендерира информацията за коментиращия в същия HTML блок като съдържанието на коментара.
- `renderLikesToRight` - премества зоната за гласувания/харесвания вдясно от коментара вместо под него.
- `renderDateBelowComment` - рендерира датата под коментара.
- `showLiveStatus` - показва чат-стил горна лента с "Live" + брой потребители над коментарите.
- `useInlineSubmitButton` - рендерира бутона за изпращане като икона вътре в композера.
- `countAboveToggle` - при `useShowCommentsToggle`, колко коментара да се рендерират над превключвателя "Show Comments".
- `preserveFeedScrollPosition` - `FastCommentsFeed` запомня своя офсет при превъртане при unmount/remount (по подразбиране true).

### FastComments Concepts

Основните понятия, които трябва да знаете за да започнете, са `tenantId` и `urlId`. `tenantId` е идентификаторът на вашия акаунт в FastComments.com. `urlId` е това, към което ще бъдат вързани нишките с коментари. Това може да е URL на страница, или идентификатор на продукт, идентификатор на статия и т.н.

### User Notifications

FastComments поддържа известия за [много сценарии](https://docs.fastcomments.com/guide-notifications.html). Известията са конфигурируеми, може да се откажете от тях глобално или на ниво известие/коментар и поддържат абонаменти на ниво страница, така че потребителите да могат да се абонират за нишки от конкретна страница или статия.

Например, възможно е да се използва Secure SSO за удостоверяване на потребителя и след това периодично да се проверява за непрочетени известия и да се изпращат на потребителя.

Вижте [примерния AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) за това как да получите и превърнете непрочетени потребителски известия.

### Gif Browser

По подразбиране не е активиран избор на изображения или GIF. Вижте [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) за това как да поддържате качване на изображения и GIF. В тази библиотека има GIF браузър, който анонимизира търсенията и изображенията, предоставени в него — просто трябва да го използвате.

### Performance

Моля, отворете ticket с пример за възпроизвеждане, включително използваното устройство, ако откриете проблеми с производителността. Производителността е приоритет във всички библиотеки на FastComments.