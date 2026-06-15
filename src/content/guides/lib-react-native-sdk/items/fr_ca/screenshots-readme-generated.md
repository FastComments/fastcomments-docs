Commentaires en fil de discussion en direct avec avatars, réponses imbriquées, votes et le composeur enrichi intégré, plus un thème sombre et un préréglage de clavardage en direct (affiché ici rendu via `react-native-web`) :

<table>
  <tr>
    <td align="center"><b>Commentaires en direct</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Commentaires en direct, thème clair"/></td>
    <td align="center"><b>Thème sombre</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Commentaires en direct, thème sombre"/></td>
    <td align="center"><b>Clavardage en direct</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Préréglage de clavardage en direct"/></td>
  </tr>
</table>

### Éditeur de texte enrichi

Cette bibliothèque utilise [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) pour l'édition de texte enrichi, qui offre une expérience WYSIWYG puissante. Le même éditeur alimente iOS, Android et le web (via `react-native-web`), de sorte que le composeur se comporte de manière cohérente sur toutes les plateformes avec une seule implémentation.

`react-native-enriched` nécessite la nouvelle architecture React Native (Fabric) sur natif (la valeur par défaut depuis RN 0.76, optionnelle sur RN 0.72-0.75), et un bundler qui résout les conditions d'exports des paquets. Ce SDK est développé et testé contre RN 0.81 / React 19. Le même éditeur fonctionne également sur le web via `react-native-web` ; la build web de l'éditeur enriched est toujours marquée expérimentale en amont.

### Widgets

Le SDK contient trois widgets, reflétant le SDK Android de FastComments :

- `FastCommentsLiveCommenting` - commentaires en fil de discussion avec votes, réponses, pagination, mentions, notifications et mises à jour en direct.
- `FastCommentsLiveChat` - un préréglage de clavardage basé sur le même moteur : messages chronologiques avec les nouveaux en bas, le composeur sous la liste, une bande d'en-tête en direct (point de connexion + nombre d'utilisateurs), historique infini chargé en remontant, auto-scroll vers les nouveaux messages, pas de votes ni de fils de réponses. Chaque préréglage peut être remplacé via `config`.
- `FastCommentsFeed` - un fil social avec composeur de post, médias, réactions, abonnements, et bannières de nouveaux posts en direct.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Thématisation

L'apparence par défaut est générée à partir d'un ensemble de tokens de design sémantiques (`FastCommentsTheme`) : couleurs, espacements, rayons, tailles de police, graisses et tailles d'avatar. Passez des remplacements partiels de tokens (typés `FastCommentsThemeOverrides`) via la prop `theme` sur n'importe quel widget et tout l'arbre de styles se re-stylise de manière cohérente :

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Le mode sombre est à un ensemble de tokens près :

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La prop `styles` accepte toujours un arbre brut `IFastCommentsStyles` pour un contrôle chirurgical. Lorsque `theme` et `styles` sont tous deux fournis, les styles explicites prennent le pas sur l'arbre thématisé ; lorsque seul `styles` est fourni, il remplace entièrement les valeurs par défaut (le comportement original, de sorte que les intégrations et skins existants ne sont pas affectés). `setupDarkModeSkin` est obsolète au profit de la prop theme.

### Options de configuration

Cette bibliothèque vise à supporter toutes les options de configuration définies dans [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tout comme l'implémentation web.

En plus de celles-ci, React Native ajoute quelques options spécifiques au SDK via `FastCommentsRNConfig` :

- `hideTopBar` - masquer la bande montrant l'utilisateur connecté / l'icône de notification située au-dessus du composeur.
- `usePressToEdit` - appuyer longuement sur un commentaire pour ouvrir son menu.
- `disableDownVoting` - masquer les boutons de vote négatif.
- `renderCommentInline` - afficher les informations du commentateur à l'intérieur du même bloc HTML que le contenu du commentaire.
- `renderLikesToRight` - déplacer la zone de vote/j'aime à droite du commentaire plutôt qu'en dessous.
- `renderDateBelowComment` - afficher la date sous le commentaire.
- `showLiveStatus` - afficher la bande d'en-tête de type chat "Live" + nombre d'utilisateurs au-dessus des commentaires.
- `useInlineSubmitButton` - afficher le bouton d'envoi comme une icône à l'intérieur du composeur.
- `countAboveToggle` - avec `useShowCommentsToggle`, combien de commentaires s'affichent au-dessus du toggle « Afficher les commentaires ».
- `preserveFeedScrollPosition` - `FastCommentsFeed` se souvient de son décalage de défilement lors de démontage/remontage (par défaut true).

### Concepts FastComments

Les principaux concepts à connaître pour commencer sont `tenantId` et `urlId`. `tenantId` est l'identifiant de votre compte FastComments.com. `urlId` correspond à l'entité à laquelle les fils de commentaires seront rattachés. Il peut s'agir d'une URL de page, d'un identifiant de produit, d'un identifiant d'article, etc.

### Notifications utilisateur

FastComments prend en charge les notifications pour [de nombreux scénarios](https://docs.fastcomments.com/guide-notifications.html). Les notifications sont configurables, peuvent être désactivées globalement ou au niveau d'une notification/commentaire, et prennent en charge les abonnements au niveau de la page afin que les utilisateurs puissent s'abonner aux fils d'une page ou d'un article spécifique.

Par exemple, il est possible d'utiliser Secure SSO pour authentifier l'utilisateur puis de sonder périodiquement les notifications non lues et de les pousser vers l'utilisateur.

Voir [l'exemple AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) pour savoir comment obtenir et traduire les notifications utilisateur non lues.

### Navigateur de GIF

Par défaut, aucune sélection d'image ou de GIF n'est activée. Voir [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) pour savoir comment prendre en charge les téléchargements d'images et de GIF. Il existe un Navigateur de GIF qui anonymise les recherches et les images fourni dans cette bibliothèque, il vous suffit de l'utiliser.

### Performances

Veuillez ouvrir un ticket avec un exemple reproduisant le problème, y compris l'appareil utilisé, si vous identifiez des problèmes de performance. Les performances sont une priorité de premier ordre pour toutes les bibliothèques FastComments.