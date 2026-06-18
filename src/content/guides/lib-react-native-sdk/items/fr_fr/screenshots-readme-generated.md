Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Commentaires en direct</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Commentaires en direct, thème clair"/></td>
    <td align="center"><b>Thème sombre</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Commentaires en direct, thème sombre"/></td>
    <td align="center"><b>Chat en direct</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Préréglage de chat en direct"/></td>
  </tr>
</table>

### Éditeur de texte enrichi

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native (the default since RN 0.76, opt-in on RN 0.72-0.75), and a bundler that resolves package `exports` conditions. This SDK is developed and tested against RN 0.81 / React 19. The same editor also runs on web through `react-native-web`; the enriched editor's web build is still marked experimental upstream.

### Widgets

Le SDK inclut trois widgets, reprenant le SDK Android FastComments :

- `FastCommentsLiveCommenting` - commentaires en fils avec votes, réponses, pagination, mentions, notifications et mises à jour en direct.
- `FastCommentsLiveChat` - un préréglage chat reposant sur le même moteur : messages chronologiques avec les nouveaux en bas, le composeur sous la liste, une bande d'en-tête en direct (point de connexion + nombre d'utilisateurs), historique infini chargé en faisant défiler vers le haut, auto-scroll vers les nouveaux messages, pas de votes ni de fils de réponses. Chaque préréglage peut être remplacé via `config`.
- `FastCommentsFeed` - un fil social avec composeur de posts, médias, réactions, abonnements, et bannières de nouveaux posts en direct.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Thématisation

The default look is generated from a set of semantic design tokens (`FastCommentsTheme`): colors, spacing, radius, font sizes, font weights, and avatar sizes. Pass partial token overrides (typed `FastCommentsThemeOverrides`) through the `theme` prop on any widget and the entire style tree restyles consistently:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Dark mode is one token set away:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La prop `styles` accepte toujours un arbre brut `IFastCommentsStyles` pour un contrôle chirurgical. Lorsque `theme` et `styles` sont fournis, les styles explicites priment sur l'arbre thématisé ; lorsque seul `styles` est fourni, il remplace entièrement les valeurs par défaut (comportement d'origine, de sorte que les intégrations et skins existants ne sont pas affectés). `setupDarkModeSkin` est déprécié au profit de la prop `theme`.

### Options de configuration

This library aims to support all configuration options defined in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), just like the web implementation.

On top of those, React Native adds a few SDK-specific options via `FastCommentsRNConfig`:

- `hideTopBar` - masquer la bande affichant l'utilisateur connecté / la cloche de notifications au-dessus du composeur.
- `usePressToEdit` - appuyer et maintenir un commentaire pour ouvrir son menu.
- `disableDownVoting` - masquer les boutons de vote négatif.
- `renderCommentInline` - afficher les informations du commentateur dans le même bloc HTML que le contenu du commentaire.
- `renderLikesToRight` - déplacer la zone de vote/like à droite du commentaire au lieu de sous celui-ci.
- `renderDateBelowComment` - afficher la date sous le commentaire.
- `showLiveStatus` - afficher la bande d'en-tête de type chat "Live" + nombre d'utilisateurs au-dessus des commentaires.
- `useInlineSubmitButton` - afficher le bouton d'envoi comme une icône à l'intérieur du composeur.
- `countAboveToggle` - avec `useShowCommentsToggle`, combien de commentaires s'affichent au-dessus du bascule "Show Comments".
- `preserveFeedScrollPosition` - `FastCommentsFeed` mémorise son offset de défilement entre démontage/remontage (true par défaut).

### Concepts de FastComments

Les concepts principaux à connaître pour démarrer sont `tenantId` et `urlId`. `tenantId` est l'identifiant de votre compte FastComments.com. `urlId` est l'endroit auquel les fils de commentaires seront rattachés. Cela peut être une URL de page, un identifiant de produit, un identifiant d'article, etc.

### Notifications utilisateur

FastComments prend en charge les notifications pour [de nombreux scénarios](https://docs.fastcomments.com/guide-notifications.html). Les notifications sont configurables, peuvent être désactivées globalement ou au niveau d'une notification/commentaire, et prennent en charge les abonnements au niveau de la page afin que les utilisateurs puissent s'abonner aux fils d'une page ou d'un article spécifique.

Par exemple, il est possible d'utiliser Secure SSO pour authentifier l'utilisateur puis de sonder périodiquement les notifications non lues et les pousser vers l'utilisateur.

Voir [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) pour savoir comment obtenir et traduire les notifications utilisateur non lues.

### Navigateur de GIFs

Par défaut, aucune sélection d'image ou de GIF n'est activée. Voir [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) pour savoir comment prendre en charge les téléchargements d'images et de GIFs. Il existe un navigateur de GIF qui anonymise les recherches et les images fourni dans cette bibliothèque, il vous suffit de l'utiliser.

### Performances

Veuillez ouvrir un ticket avec un exemple reproductible, y compris l'appareil utilisé, si vous identifiez des problèmes de performance. Les performances sont une priorité dans toutes les bibliothèques FastComments.