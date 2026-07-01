Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Commenting</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live commenting, light theme"/></td>
    <td align="center"><b>Dark Theme</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live commenting, dark theme"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Rich Text Editor

Ця бібліотека використовує [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) для редагування форматованого тексту, який забезпечує потужний WYSIWYG‑досвід. Той самий редактор живе в iOS, Android та веб (через `react-native-web`), тому композер поводиться послідовно на всіх платформах з єдиною реалізацією.

`react-native-enriched` вимагає нову архітектуру React Native (Fabric) на нативних платформах (за замовчуванням з RN 0.76, опціонально в RN 0.72‑0.75), а також збирач, який розв’язує умови `exports` у пакеті. Цей SDK розроблено та протестовано проти RN 0.81 / React 19. Той самий редактор також працює в інтернеті через `react-native-web`; веб‑збірка enriched‑редактора ще позначена як експериментальна в оригіналі.

### Widgets

SDK постачається з трьома віджетами, які відповідають Android SDK FastComments:

- `FastCommentsLiveCommenting` – потокові коментарі з голосуванням, відповідями, пагінацією, згадками, сповіщеннями та живими оновленнями.
- `FastCommentsLiveChat` – чат‑preset на тому ж движку: хронологічні повідомлення з новими внизу, композер під списком, живий заголовок‑смужка (точка підключення + кількість користувачів), нескінченна історія, завантажувана прокручуванням вгору, авто‑прокрутка до нових повідомлень, без голосування або гілок відповідей. Кожен preset можна перевизначити за допомогою `config`.
- `FastCommentsFeed` – соціальна стрічка з композером посту, медіа, реакціями, підписками та банерами нових постів у реальному часі.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Типовий вигляд генерується з набору семантичних токенів дизайну (`FastCommentsTheme`): кольори, відступи, радіуси, розміри шрифтів, ваги шрифтів та розміри аватарок. Передайте часткові перевизначення токенів (типу `FastCommentsThemeOverrides`) через проп `theme` у будь‑якому віджеті, і вся дерево стилів буде оновлено послідовно:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Темний режим – це лише інший набір токенів:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Проп `styles` все ще приймає чисте дерево `IFastCommentsStyles` для точного контролю. Коли `theme` і `styles` передані одночасно, явні стилі мають перевагу над темою; коли передано лише `styles`, вони повністю замінюють значення за замовчуванням (початкова поведінка, тому існуючі інтеграції та скіни не зміняться). `setupDarkModeSkin` вважається застарілим на користь пропу `theme`.

### Configuration Options

Бібліотека прагне підтримувати всі параметри конфігурації, визначені в [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), так само, як і веб‑реалізація.

Понад це, React Native додає кілька специфічних для SDK параметрів через `FastCommentsRNConfig`:

- `hideTopBar` – приховати смужку з інформацією про залогіненого користувача / сповіщеннями над композером.
- `usePressToEdit` – натиснути‑утримати коментар, щоб відкрити його меню.
- `disableDownVoting` – приховати кнопки голосування «проти».
- `renderCommentInline` – відображати інформацію про коментатора всередині того ж HTML‑блоку, що й вміст коментаря.
- `renderLikesToRight` – перемістити область голосування/лайків праворуч від коментаря замість під ним.
- `renderDateBelowComment` – розташувати дату під коментарем.
- `showLiveStatus` – показати смужку‑заголовок «Live» + кількість користувачів над коментарями, як у чаті.
- `useInlineSubmitButton` – відображати кнопку відправки як іконку всередині композера.
- `countAboveToggle` – разом з `useShowCommentsToggle`, скільки коментарів показувати над перемикачем «Show Comments».
- `preserveFeedScrollPosition` – `FastCommentsFeed` запам’ятовує позицію прокрутки між розмонтуванням/монтуванням (за замовчуванням true).

### FastComments Concepts

Основні концепції, які треба знати перед стартом, – це `tenantId` та `urlId`. `tenantId` – ваш ідентифікатор облікового запису на FastComments.com. `urlId` – прив’язка до конкретної сторінки або продукту, статті тощо, до якої будуть прив’язуватись потокові коментарі.

### Localization

Весь користувацький текст у цих віджетах (мітки кнопок, підказки, порожні стани, відносні дати типу «5 хвилин тому», повідомлення про помилки тощо) **генерується на сервері**. Компоненти не мають жорстко закодованих англійських рядків; вони відображають переклади, які FastComments повертає для запитаної локалі.

Щоб задати локаль, встановіть `locale` у вашій конфігурації:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Якщо `locale` не вказано, FastComments використовує мову за замовчуванням орендаря.

**Редагування тексту:** переклади керуються у вашій панелі FastComments, а не в цьому SDK. Щоб змінити формулювання, перевизначте стандартний копірайт або додайте мову, відредагуйте переклади вашого облікового запису в панелі – зміни підхоплюються віджетами автоматично, без випуску нових збірок застосунку. SDK не постачається з англійськими резервними варіантами, тому будь‑який ключ, який ви очистите в панелі, буде відображений порожнім; залишайте заповнені ключі для кожної підтримуваної локалі.

### User Notifications

FastComments підтримує сповіщення для [багатьох сценаріїв](https://docs.fastcomments.com/guide-notifications.html). Сповіщення налаштовуються, їх можна глобально або за рівнем сповіщення/коментаря відключити, а також підтримуються підписки на рівні сторінки, щоб користувачі могли підписатися на потоки певної сторінки або статті.

Наприклад, можливо використати Secure SSO для автентифікації користувача, а потім періодично опитувати сервер щодо непрочитаних сповіщень і надсилати їх користувачу.

Дивіться [приклад AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) для того, як отримувати та відображати непрочитані сповіщення користувача.

### Gif Browser

За замовчуванням вибір зображень або GIF‑ів вимкнено. Дивіться [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) для того, як підтримувати завантаження зображень і GIF‑ів. У бібліотеці є Gif‑Browser, який анонімізує пошук та зображення, його достатньо просто використати.

### Performance

Будь ласка, відкривайте тикет із прикладом для відтворення, включно з використаним пристроєм, якщо ви виявите будь‑які проблеми продуктивності. Продуктивність є першорядним критерієм усіх бібліотек FastComments.