Commentaires en direct avec fils de discussion, avatars, réponses imbriquées, votes et éditeur de texte enrichi intégré, plus un thème sombre et un preset de chat en direct (rendu ici via `react-native-web`) :

<table>
  <tr>
    <td align="center"><b>Commentaires en direct</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Commentaires en direct, thème clair"/></td>
    <td align="center"><b>Thème sombre</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Commentaires en direct, thème sombre"/></td>
    <td align="center"><b>Chat en direct</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Preset de chat en direct"/></td>
  </tr>
</table>

### Éditeur de texte enrichi

Cette bibliothèque utilise [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) pour l'édition de texte enrichi, qui offre une expérience d'édition WYSIWYG puissante. Le même éditeur alimente iOS, Android et le web (via `react-native-web`), de sorte que le composeur se comporte de manière cohérente sur toutes les plateformes avec une seule implémentation.

`react-native-enriched` nécessite la New Architecture de React Native (Fabric) sur natif (par défaut depuis RN 0.76, optionnel sur RN 0.72-0.75), et un bundler qui résout les conditions `exports` des packages. Ce SDK est développé et testé avec RN 0.81 / React 19. Le même éditeur fonctionne également sur le web via `react-native-web` ; la version web de l'éditeur enrichi est encore marquée comme expérimentale en amont.

### Widgets

Le SDK fournit trois widgets, à l'image du SDK Android FastComments :

- `FastCommentsLiveCommenting` - commentaires avec fils de discussion, votes, réponses, pagination, mentions, notifications et mises à jour en direct.
- `FastCommentsLiveChat` - un preset de chat sur le même moteur : messages chronologiques avec les nouveaux en bas, composeur sous la liste, bandeau d'en-tête en direct (point de connexion + nombre d'utilisateurs), historique infini chargé en défilant vers le haut, défilement automatique vers les nouveaux messages, sans votes ni fils de réponses. Chaque preset peut être remplacé via `config`.
- `FastCommentsFeed` - un fil social avec composeur de publications, médias, réactions, abonnements et bannières de nouvelles publications en direct.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Thématisation

L'apparence par défaut est générée à partir d'un ensemble de jetons de design sémantiques (`FastCommentsTheme`) : couleurs, espacements, rayons, tailles de police, graisses de police et tailles d'avatar. Passez des substitutions partielles de jetons (typées `FastCommentsThemeOverrides`) via la prop `theme` sur n'importe quel widget et l'ensemble de l'arborescence de style est restylé de manière cohérente :

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Le mode sombre n'est qu'à un jeu de jetons :

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La prop `styles` accepte toujours un arbre `IFastCommentsStyles` brut pour un contrôle précis. Lorsque `theme` et `styles` sont fournis tous les deux, les styles explicites l'emportent sur l'arbre thématisé ; lorsque seul `styles` est fourni, il remplace entièrement les valeurs par défaut (le comportement d'origine, afin que les intégrations et skins existants ne soient pas affectés). `setupDarkModeSkin` est déprécié au profit de la prop `theme`.

### Options de configuration

Cette bibliothèque vise à prendre en charge toutes les options de configuration définies dans [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tout comme l'implémentation web.

En plus de celles-ci, React Native ajoute quelques options spécifiques au SDK via `FastCommentsRNConfig` :

- `hideTopBar` - masque la bande utilisateur connecté / cloche de notifications affichée au-dessus du composeur.
- `usePressToEdit` - appuyer longuement sur un commentaire pour ouvrir son menu.
- `disableDownVoting` - masque les boutons de vote négatif.
- `renderCommentInline` - affiche les informations du commentateur dans le même bloc HTML que le contenu du commentaire.
- `renderLikesToRight` - déplace la zone de vote/j'aime à droite du commentaire au lieu de dessous.
- `renderDateBelowComment` - affiche la date sous le commentaire.
- `showLiveStatus` - affiche le bandeau d'en-tête de type chat « Live » + nombre d'utilisateurs au-dessus des commentaires.
- `useInlineSubmitButton` - affiche le bouton d'envoi sous forme d'icône dans le composeur.
- `countAboveToggle` - avec `useShowCommentsToggle`, combien de commentaires s'affichent au-dessus du bouton « Show Comments ».
- `preserveFeedScrollPosition` - `FastCommentsFeed` mémorise sa position de défilement entre un démontage et un remontage (true par défaut).

### Concepts FastComments

Les principaux concepts à connaître pour démarrer sont `tenantId` et `urlId`. `tenantId` est l'identifiant de votre compte FastComments.com. `urlId` détermine à quoi seront rattachés les fils de commentaires. Il peut s'agir de l'URL d'une page, d'un identifiant de produit, d'un identifiant d'article, etc.

### Localisation

Tous les textes visibles par l'utilisateur dans ces widgets (libellés de boutons, placeholders, états vides, dates relatives comme « il y a 5 minutes », messages d'erreur, etc.) sont **pilotés par le serveur**. Les composants ne codent pas en dur les chaînes anglaises ; ils affichent les traductions que FastComments fournit pour la locale demandée.

Pour demander une locale, définissez `locale` dans votre config :

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Lorsqu'aucune `locale` n'est définie, FastComments sert la langue par défaut du tenant.

**Modification du texte :** les traductions sont gérées dans votre tableau de bord FastComments, pas dans ce SDK. Pour modifier la formulation, remplacer le texte par défaut ou ajouter une langue, modifiez les traductions de votre compte dans le tableau de bord - la modification est prise en compte par les widgets automatiquement, sans publication d'application. Le SDK n'embarque aucun repli en anglais, donc toute clé que vous videz dans le tableau de bord s'affiche vide ; gardez les clés renseignées pour chaque locale que vous prenez en charge.

### Notifications utilisateur

FastComments prend en charge les notifications pour [de nombreux scénarios](https://docs.fastcomments.com/guide-notifications.html). Les notifications sont configurables, peuvent être désactivées globalement ou au niveau d'une notification/commentaire, et prennent en charge les abonnements au niveau de la page afin que les utilisateurs puissent s'abonner aux fils d'une page ou d'un article spécifique.

Par exemple, il est possible d'utiliser le SSO sécurisé pour authentifier l'utilisateur, puis de sonder périodiquement les notifications non lues et de les pousser vers l'utilisateur.

Voir [l'exemple AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) pour savoir comment récupérer et traduire les notifications utilisateur non lues.

### Navigateur de GIF

Par défaut, aucune sélection d'image ou de gif n'est activée. Voir [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) pour savoir comment prendre en charge le téléversement d'images et de gifs. Il existe un navigateur de GIF qui anonymise les recherches et les images fournies dans cette bibliothèque, il vous suffit de l'utiliser.

### Performances

Veuillez ouvrir un ticket avec un exemple de reproduction, incluant l'appareil utilisé, si vous identifiez des problèmes de performance. La performance est un citoyen de première classe dans toutes les bibliothèques FastComments.