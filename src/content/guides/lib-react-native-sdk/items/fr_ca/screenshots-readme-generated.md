Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Commentaires en direct</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Commentaires en direct, thème clair"/></td>
    <td align="center"><b>Thème sombre</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Commentaires en direct, thème sombre"/></td>
    <td align="center"><b>Chat en direct</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Préréglage de chat en direct"/></td>
  </tr>
</table>

### Rich Text Editor

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native (the default since RN 0.76, opt-in on RN 0.72-0.75), and a bundler that resolves package `exports` conditions. This SDK is developed and tested against RN 0.81 / React 19. The same editor also runs on web through `react-native-web`; the enriched editor's web build is still marked experimental upstream.

### Widgets

The SDK ships three widgets, mirroring the FastComments Android SDK:

- `FastCommentsLiveCommenting` – commentaires en fil de discussion avec votes, réponses, pagination, mentions, notifications et mises à jour en temps réel.
- `FastCommentsLiveChat` – un préréglage de chat basé sur le même moteur : messages chronologiques avec les nouveaux en bas, le compositeur sous la liste, une bandeau d’en‑tête en direct (point de connexion + nombre d’utilisateurs), historique infini chargé en faisant défiler vers le haut, défilement automatique aux nouveaux messages, aucun vote ni fil de réponses. Chaque préréglage peut être remplacé via `config`.
- `FastCommentsFeed` – un fil social avec compositeur de publication, médias, réactions, suivis et bannières de nouvelles publications en direct.

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

- `hideTopBar` – masquer la bandeau d’utilisateur connecté / cloche de notification affichée au-dessus du compositeur.
- `usePressToEdit` – appuyer et maintenir un commentaire pour ouvrir son menu.
- `disableDownVoting` – masquer les boutons de vote négatif.
- `renderCommentInline` – rendre les informations du commentateur à l’intérieur du même bloc HTML que le contenu du commentaire.
- `renderLikesToRight` – déplacer la zone de vote/like à droite du commentaire au lieu de dessous.
- `renderDateBelowComment` – afficher la date sous le commentaire.
- `showLiveStatus` – afficher la bandeau d’en‑tête « Live » + compte d’utilisateurs de style chat au-dessus des commentaires.
- `useInlineSubmitButton` – rendre le bouton d’envoi comme une icône à l’intérieur du compositeur.
- `countAboveToggle` – avec `useShowCommentsToggle`, combien de commentaires s’affichent au‑dessus de l’interrupteur « Afficher les commentaires ».
- `preserveFeedScrollPosition` – `FastCommentsFeed` se souvient de son offset de défilement entre les démontages/remontages (true par défaut).

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
account in the dashboard – the change is picked up by the widgets automatically with no app
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