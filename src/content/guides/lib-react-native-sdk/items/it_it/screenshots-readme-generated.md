---
Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Commenti in tempo reale</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Commenti in tempo reale, tema chiaro"/></td>
    <td align="center"><b>Tema scuro</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Commenti in tempo reale, tema scuro"/></td>
    <td align="center"><b>Chat in tempo reale</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Preset chat in tempo reale"/></td>
  </tr>
</table>

### Editor di testo ricco

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native (the default since RN 0.76, opt-in on RN 0.72-0.75), and a bundler that resolves package `exports` conditions. This SDK is developed and tested against RN 0.81 / React 19. The same editor also runs on web through `react-native-web`; the enriched editor's web build is still marked experimental upstream.

### Widget

The SDK ships three widgets, mirroring the FastComments Android SDK:

- `FastCommentsLiveCommenting` - threaded commenting with votes, replies, pagination, mentions, notifications, and live updates.
- `FastCommentsLiveChat` - a chat preset over the same engine: chronological messages with new ones at the bottom, the composer below the list, a live header strip (connection dot + user count), infinite history loaded by scrolling up, auto-scroll to new messages, no votes or reply threading. Every preset can be overridden via `config`.
- `FastCommentsFeed` - a social feed with post composer, media, reactions, follows, and live new-post banners.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Temi

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

### Opzioni di configurazione

This library aims to support all configuration options defined in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), just like the web implementation.

On top of those, React Native adds a few SDK-specific options via `FastCommentsRNConfig`:

- `hideTopBar` - nascondere la striscia con l'utente autenticato / l'icona delle notifiche mostrata sopra il composer.
- `usePressToEdit` - premi e tieni premuto su un commento per aprire il suo menu.
- `disableDownVoting` - nascondere i pulsanti di voto negativo.
- `renderCommentInline` - renderizzare le informazioni del commentatore all'interno dello stesso blocco HTML del contenuto del commento.
- `renderLikesToRight` - spostare l'area voto/like a destra del commento invece che sotto di esso.
- `renderDateBelowComment` - visualizzare la data sotto il commento.
- `showLiveStatus` - mostrare la striscia header in stile chat "Live" + numero utenti sopra i commenti.
- `useInlineSubmitButton` - visualizzare il pulsante di invio come icona all'interno del composer.
- `countAboveToggle` - con `useShowCommentsToggle`, quanti commenti vengono mostrati sopra il toggle "Show Comments".
- `preserveFeedScrollPosition` - `FastCommentsFeed` ricorda il suo offset di scorrimento attraverso unmount/remount (default true).

### Concetti principali di FastComments

The main concepts to be aware of to get started are `tenantId` and `urlId`. `tenantId` is your FastComments.com account identifier. `urlId` is where comment threads
will be tied to. This could be a page URL, or a product id, an article id, etc.

### Notifiche utente

FastComments supports notifications for [many scenarios](https://docs.fastcomments.com/guide-notifications.html). Notifications are configurable,
can be opted-out globally or at a notification/comment level, and supports page-level subscriptions so that users can subscribe to threads of a
specific page or article.

For example, it is possible to use Secure SSO to authenticate the user and then periodically poll for unread notifications and push them to the user.

See [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) for how to get and translate unread user notifications.

### Browser GIF

By default, no image or gif selection is enabled. See [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) for how
to support image and gif uploads. There is a Gif Browser that anonymizes searches and images provided in this library, you simply have to use it.

### Prestazioni

Please open a ticket with an example to reproduce, including device used, if you identify any performance problems. Performance is a first-class citizen
of all FastComments libraries.
---