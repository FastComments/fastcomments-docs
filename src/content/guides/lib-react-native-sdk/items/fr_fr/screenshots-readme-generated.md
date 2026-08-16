Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Commentaire en direct</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Commentaire en direct, thème clair"/></td>
    <td align="center"><b>Thème sombre</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Commentaire en direct, thème sombre"/></td>
    <td align="center"><b>Chat en direct</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Préréglage de chat en direct"/></td>
  </tr>
</table>

### Éditeur de texte enrichi

Cette bibliothèque utilise [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) pour l'édition de texte enrichi, offrant une expérience d'édition WYSIWYG puissante. Le même éditeur alimente iOS, Android et le web (via `react-native-web`), de sorte que le compositeur se comporte de manière cohérente sur chaque plateforme avec une implémentation unique.

`react-native-enriched` nécessite la nouvelle architecture de React Native (Fabric) sur le natif (défaut depuis RN 0.76, optionnel sur RN 0.72-0.75), ainsi qu'un bundler qui résout les conditions `exports` du paquet. Ce SDK est développé et testé avec RN 0.81 / React 19. Le même éditeur fonctionne également sur le web via `react-native-web` ; la version web de l'éditeur enrichi est encore marquée comme expérimentale en amont.

### Widgets

Le SDK fournit trois widgets, reproduisant le SDK Android de FastComments :

- `FastCommentsLiveCommenting` - commentaire en fil de discussion avec votes, réponses, pagination, mentions, notifications et mises à jour en temps réel.
- `FastCommentsLiveChat` - un préréglage de chat basé sur le même moteur : messages chronologiques avec les nouveaux en bas, le compositeur sous la liste, une bandeau d'en-tête en direct (point de connexion + nombre d'utilisateurs), historique infini chargé en faisant défiler vers le haut, défilement automatique vers les nouveaux messages, sans votes ni réponses en fil. Chaque préréglage peut être remplacé via `config`.
- `FastCommentsFeed` - un fil social avec compositeur de publication, médias, réactions, abonnements et bannières en direct pour les nouvelles publications.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Thématisation

L'apparence par défaut est générée à partir d'un ensemble de jetons de conception sémantiques (`FastCommentsTheme`) : couleurs, espacements, rayons, tailles de police, graisses de police et tailles d'avatar. Passez des surcharges partielles de jetons (de type `FastCommentsThemeOverrides`) via la propriété `theme` sur n'importe quel widget et l'arbre de styles complet sera rethématisé de manière cohérente :

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Le mode sombre n'est qu'un autre jeu de jetons :

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La propriété `styles` accepte toujours un arbre brut `IFastCommentsStyles` pour un contrôle précis. Lorsque `theme` et `styles` sont tous deux fournis, les styles explicites prévalent sur l'arbre thématisé ; lorsque seul `styles` est fourni, il remplace entièrement les valeurs par défaut (comportement d'origine, ainsi les intégrations et skins existants ne sont pas affectés). `setupDarkModeSkin` est obsolète au profit de la propriété `theme`.

### Options de configuration

Cette bibliothèque vise à prendre en charge toutes les options de configuration définies dans [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tout comme l'implémentation web.

En plus de celles-ci, React Native ajoute quelques options spécifiques au SDK via `FastCommentsRNConfig` :

- `hideTopBar` - masquer la bandeau de l'utilisateur connecté / cloche de notification affiché au-dessus du compositeur.
- `usePressToEdit` - appuyer longuement sur un commentaire pour ouvrir son menu.
- `disableDownVoting` - masquer les boutons de vote négatif.
- `renderCommentInline` - rendre les informations du commentateur à l'intérieur du même bloc HTML que le contenu du commentaire.
- `renderLikesToRight` - déplacer la zone de vote/like à droite du commentaire au lieu de dessous.
- `renderDateBelowComment` - afficher la date sous le commentaire.
- `showLiveStatus` - afficher la bandeau d'en-tête de style chat « Live » + compteur d'utilisateurs au-dessus des commentaires.
- `useInlineSubmitButton` - rendre le bouton d'envoi sous forme d'icône à l'intérieur du compositeur.
- `countAboveToggle` - avec `useShowCommentsToggle`, combien de commentaires sont affichés au-dessus du bascule « Show Comments ».
- `preserveFeedScrollPosition` - `FastCommentsFeed` se souvient de son décalage de défilement entre les démontages/remontages (true par défaut).

### Concepts FastComments

Les concepts principaux à connaître pour démarrer sont `tenantId` et `urlId`. `tenantId` est l'identifiant de votre compte FastComments.com. `urlId` est l'élément auquel les fils de commentaires seront associés. Cela peut être une URL de page, un identifiant de produit, un identifiant d'article, etc.

### Localisation

Tout le texte destiné aux utilisateurs dans ces widgets (étiquettes de boutons, espaces réservés, états vides, dates relatives comme « il y a 5 minutes », messages d'erreur, etc.) est **géré par le serveur**. Les composants ne codent pas en dur les chaînes en anglais ; ils affichent les traductions que FastComments fournit pour la locale demandée.

Pour demander une locale, définissez `locale` dans votre configuration :

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Lorsque aucune `locale` n'est définie, FastComments utilise la langue par défaut du locataire.

**Modification du texte :** les traductions sont gérées dans votre tableau de bord FastComments, pas dans ce SDK. Pour modifier le libellé, remplacez le texte par défaut, ou ajoutez une langue, éditez les traductions de votre compte dans le tableau de bord — le changement est automatiquement pris en compte par les widgets sans nécessiter de publication d'application. Le SDK ne fournit aucun texte de secours en anglais, ainsi toute clé que vous videz dans le tableau de bord s'affiche vide ; conservez les clés renseignées pour chaque locale prise en charge.

### Notifications utilisateur

FastComments prend en charge les notifications pour [de nombreux scénarios](https://docs.fastcomments.com/guide-notifications.html). Les notifications sont configurables, peuvent être désactivées globalement ou au niveau d'une notification/commentaire, et supportent les abonnements au niveau de la page afin que les utilisateurs puissent s'abonner aux fils d'une page ou d'un article spécifique.

Par exemple, il est possible d'utiliser Secure SSO pour authentifier l'utilisateur puis d'interroger périodiquement les notifications non lues et de les pousser à l'utilisateur.

Voir [l'exemple AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) pour savoir comment récupérer et traduire les notifications utilisateur non lues.

### Navigateur Gif

Par défaut, aucune sélection d'image ou de gif n'est activée. Consultez [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) pour savoir comment prendre en charge les téléchargements d'images et de gifs. Il existe un Navigateur Gif qui anonymise les recherches et les images fournies dans cette bibliothèque, il suffit de l'utiliser.

### Performance

Veuillez ouvrir un ticket avec un exemple à reproduire, incluant l'appareil utilisé, si vous identifiez des problèmes de performance. La performance est une priorité de toutes les bibliothèques FastComments.