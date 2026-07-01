Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Commenting</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Живо коментиране, светла тема"/></td>
    <td align="center"><b>Dark Theme</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Живо коментиране, тъмна тема"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Предварително зададен жив чат"/></td>
  </tr>
</table>

### Rich Text Editor

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native (the default since RN 0.76, opt-in on RN 0.72-0.75), and a bundler that resolves package `exports` conditions. This SDK is developed and tested against RN 0.81 / React 19. The same editor also runs on web through `react-native-web`; the enriched editor's web build is still marked experimental upstream.

### Widgets

The SDK ships three widgets, mirroring the FastComments Android SDK:

- `FastCommentsLiveCommenting` - нишково коментиране с гласове, отговори, пагинация, споменавания, известия и живи актуализации.
- `FastCommentsLiveChat` - чат предварително зададен върху същия двигател: хронологични съобщения с нови в дъното, съставчик под списъка, живо заглавно лентено (точка за връзка + брой потребители), безкрайна история, зареждана при скролиране нагоре, автоматично скролиране към нови съобщения, без гласове или нишково отговаряне. Всеки предварителен режим може да бъде презаписан чрез `config`.
- `FastCommentsFeed` - социална хранителна лента с съставчик за публикации, медия, реакции, следвания и живи банери за нови публикации.

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

- `hideTopBar` - скриване на лентата с влезлия потребител / известия над съставчика.
- `usePressToEdit` - натискане‑и‑задържане върху коментар за отваряне на менюто му.
- `disableDownVoting` - скриване на бутоните за отрицателен глас.
- `renderCommentInline` - рендериране на информация за коментиращия в същия HTML блок като съдържанието на коментара.
- `renderLikesToRight` - местене на зоната за глас/харесване вдясно от коментара вместо под него.
- `renderDateBelowComment` - рендериране на датата под коментара.
- `showLiveStatus` - показване на чат‑стиловия „Live“ + лентата с броя потребители над коментарите.
- `useInlineSubmitButton` - рендериране на бутона за изпращане като икона вътре в съставчика.
- `countAboveToggle` - с `useShowCommentsToggle`, колко коментара се рендерират над превключвателя „Show Comments“.
- `preserveFeedScrollPosition` - `FastCommentsFeed` запомня позицията си при раз‑ и монтиране (по подразбиране true).

### FastComments Concepts

The main concepts to be aware of to get started are `tenantId` and `urlId`. `tenantId` is your FastComments.com account identifier. `urlId` is where comment threads
will be tied to. This could be a page URL, or a product id, an article id, etc.

### Localization

All user-facing text in these widgets (button labels, placeholders, empty states, relative
dates like "5 minutes ago", error messages, etc.) is **server-driven**. The components do not
hard-code English strings; they render the translations FastComments serves for the requested
locale.

To request a locale, set `locale` in your config:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

When no `locale` is set, FastComments serves the tenant's default language.

**Editing the text:** translations are managed in your FastComments dashboard, not in this SDK.
To change wording, override the default copy, or add a language, edit the translations for your
account in the dashboard - the change is picked up by the widgets automatically with no app
release required. The SDK ships no English fallbacks, so any key you blank out in the dashboard
renders empty; keep the keys populated for every locale you support.

### User Notifications

FastComments supports notifications for [many scenarios](https://docs.fastcomments.com/guide-notifications.html). Notifications are configurable,
can be opted-out globally or at a notification/comment level, and supports page-level subscriptions so that users can subscribe to threads of a
specific page or article.

For example, it is possible to use Secure SSO to authenticate the user and then periodically poll for unread notifications and push them to the user.

See [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) for how to get and translate unread user notifications.

### Gif Browser

By default, no image or gif selection is enabled. See [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) for how
to support image and gif uploads. There is a Gif Browser that anonymizes searches and images provided in this library, you simply have to use it.

### Performance

Please open a ticket with an example to reproduce, including device used, if you identify any performance problems. Performance is a first-class citizen
of all FastComments libraries.