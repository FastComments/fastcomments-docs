Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Живое комментирование</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Живое комментирование, светлая тема"/></td>
    <td align="center"><b>Тёмная тема</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Живое комментирование, тёмная тема"/></td>
    <td align="center"><b>Живой чат</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Пресет живого чата"/></td>
  </tr>
</table>

### Rich Text Editor

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native (the default since RN 0.76, opt-in on RN 0.72-0.75), and a bundler that resolves package `exports` conditions. This SDK is developed and tested against RN 0.81 / React 19. The same editor also runs on web through `react-native-web`; the enriched editor's web build is still marked experimental upstream.

### Widgets

The SDK ships three widgets, mirroring the FastComments Android SDK:

- `FastCommentsLiveCommenting` - ветвлённое комментирование с голосованиями, ответами, пагинацией, упоминаниями, уведомлениями и живыми обновлениями.
- `FastCommentsLiveChat` - пресет чата на том же движке: хронологические сообщения с новыми внизу, композер под списком, живая заголовочная полоса (точка соединения + количество пользователей), бесконечная история, загружаемая при прокрутке вверх, авто‑прокрутка к новым сообщениям, без голосований и ветвления ответов. Каждый пресет можно переопределить через `config`.
- `FastCommentsFeed` - социальная лента с композером постов, медиа, реакциями, подписками и живыми баннерами новых постов.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

The default look is generated from a set of semantic design tokens (`FastCommentsTheme`): colors, spacing, radius, font sizes, font weights, and avatar sizes. Pass partial token overrides (typed `FastCommentsThemeOverrides`) through the `theme` prop on any widget and the entire style tree restyles consistently:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Dark mode is one token set away:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

The `styles` prop still accepts a raw `IFastCommentsStyles` tree for surgical control. When `theme` and `styles` are both provided, the explicit styles win over the themed tree; when only `styles` is provided, it replaces the defaults entirely (the original behavior, so existing integrations and skins are unaffected). `setupDarkModeSkin` is deprecated in favor of the `theme` prop.

### Configuration Options

This library aims to support all configuration options defined in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), just like the web implementation.

On top of those, React Native adds a few SDK-specific options via `FastCommentsRNConfig`:

- `hideTopBar` - скрыть полосу с залогиненным пользователем / иконкой уведомлений, отображаемую над композером.
- `usePressToEdit` - нажать и удерживать комментарий, чтобы открыть его меню.
- `disableDownVoting` - скрыть кнопки голосования «против».
- `renderCommentInline` - отображать информацию о комментаторе внутри того же HTML‑блока, что и содержимое комментария.
- `renderLikesToRight` - перенести область голосования/лайков вправо от комментария вместо снизу.
- `renderDateBelowComment` - отображать дату под комментарием.
- `showLiveStatus` - показывать в виде чата заголовочную полосу «Live» + количество пользователей над комментариями.
- `useInlineSubmitButton` - отображать кнопку отправки в виде иконки внутри композера.
- `countAboveToggle` - при `useShowCommentsToggle` количество комментариев, отображаемых над переключателем «Показать комментарии».
- `preserveFeedScrollPosition` - `FastCommentsFeed` запоминает своё смещение прокрутки при размонтировании/монтировании (по умолчанию true).

### FastComments Concepts

The main concepts to be aware of to get started are `tenantId` and `urlId`. `tenantId` is your FastComments.com account identifier. `urlId` is where comment threads will be tied to. This could be a page URL, or a product id, an article id, etc.

### Localization

All user-facing text in these widgets (button labels, placeholders, empty states, relative dates like "5 minutes ago", error messages, etc.) is **server-driven**. The components do not hard-code English strings; they render the translations FastComments serves for the requested locale.

To request a locale, set `locale` in your config:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

When no `locale` is set, FastComments serves the tenant's default language.

**Editing the text:** translations are managed in your FastComments dashboard, not in this SDK. To change wording, override the default copy, or add a language, edit the translations for your account in the dashboard - the change is picked up by the widgets automatically with no app release required. The SDK ships no English fallbacks, so any key you blank out in the dashboard renders empty; keep the keys populated for every locale you support.

### User Notifications

FastComments supports notifications for [many scenarios](https://docs.fastcomments.com/guide-notifications.html). Notifications are configurable, can be opted-out globally or at a notification/comment level, and supports page-level subscriptions so that users can subscribe to threads of a specific page or article.

For example, it is possible to use Secure SSO to authenticate the user and then periodically poll for unread notifications and push them to the user.

See [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) for how to get and translate unread user notifications.

### Gif Browser

By default, no image or gif selection is enabled. See [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) for how to support image and gif uploads. There is a Gif Browser that anonymizes searches and images provided in this library, you simply have to use it.

### Performance

Please open a ticket with an example to reproduce, including device used, if you identify any performance problems. Performance is a first-class citizen of all FastComments libraries.