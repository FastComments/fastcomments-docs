Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Commenting</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live commenting, light theme"/></td>
    <td align="center"><b>Dark Theme</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live commenting, dark theme"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Éditeur de texte enrichi

Cette bibliothèque utilise [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) pour l'édition de texte enrichi, ce qui offre une expérience d'édition WYSIWYG puissante. Le même éditeur alimente iOS, Android et le web (via `react-native-web`), de sorte que le compositeur se comporte de manière cohérente sur toutes les plateformes avec une seule implémentation.

`react-native-enriched` nécessite la nouvelle architecture de React Native (Fabric) sur natif (le défaut depuis RN 0.76, optionnel sur RN 0.72-0.75), et un bundler qui résout les conditions `exports` du paquet. Ce SDK est développé et testé avec RN 0.81 / React 19. Le même éditeur fonctionne également sur le web via `react-native-web` ; la version web de l'éditeur enrichi est encore marquée comme expérimentale en amont.

### Widgets

Le SDK fournit trois widgets, reproduisant le SDK Android FastComments :

- `FastCommentsLiveCommenting` - commentaires en fil de discussion avec votes, réponses, pagination, mentions, notifications et mises à jour en direct.
- `FastCommentsLiveChat` - un préréglage de chat basé sur le même moteur : messages chronologiques avec les nouveaux en bas, le compositeur sous la liste, une bandeau d’en‑tête live (point de connexion + nombre d’utilisateurs), historique infini chargé en faisant défiler vers le haut, défilement automatique vers les nouveaux messages, aucun vote ni fil de réponses. Chaque préréglage peut être remplacé via `config`.
- `FastCommentsFeed` - un fil social avec compositeur de publication, médias, réactions, abonnements et bannières « nouvelle publication en direct ».

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Thématisation

L’apparence par défaut est générée à partir d’un ensemble de tokens de conception sémantiques (`FastCommentsTheme`) : couleurs, espacements, rayons, tailles de police, graisses de police et tailles d’avatar. Passez des surcharges partielles de tokens (de type `FastCommentsThemeOverrides`) via la prop `theme` sur n’importe quel widget et l’arbre de styles complet sera restylé de manière cohérente :

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Le mode sombre n’est qu’un jeu de tokens de différence :

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La prop `styles` accepte toujours un arbre `IFastCommentsStyles` brut pour un contrôle chirurgical. Lorsque `theme` et `styles` sont tous les deux fournis, les styles explicites prévalent sur l’arbre thématisé ; lorsque seule `styles` est fournie, elle remplace entièrement les valeurs par défaut (comportement d’origine, les intégrations et skins existants restent donc inchangés). `setupDarkModeSkin` est obsolète au profit de la prop `theme`.

### Options de configuration

Cette bibliothèque vise à prendre en charge toutes les options de configuration définies dans [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tout comme l’implémentation web.

En plus de celles‑ci, React Native ajoute quelques options spécifiques au SDK via `FastCommentsRNConfig` :

- `hideTopBar` - masque la bandeau d’utilisateur connecté / cloche de notification affichée au-dessus du compositeur.
- `usePressToEdit` - appuyer longuement sur un commentaire pour ouvrir son menu.
- `disableDownVoting` - masque les boutons de vote négatif.
- `renderCommentInline` - rend les informations du commentateur à l’intérieur du même bloc HTML que le contenu du commentaire.
- `renderLikesToRight` - déplace la zone de vote/like à droite du commentaire au lieu de dessous.
- `renderDateBelowComment` - rend la date sous le commentaire.
- `showLiveStatus` - affiche la bandeau d’en‑tête de type chat « Live » + nombre d’utilisateurs au‑dessus des commentaires.
- `useInlineSubmitButton` - rend le bouton d’envoi sous forme d’icône à l’intérieur du compositeur.
- `countAboveToggle` - avec `useShowCommentsToggle`, indique combien de commentaires sont rendus au‑dessus du bascule « Show Comments ».
- `preserveFeedScrollPosition` - `FastCommentsFeed` se souvient de son offset de défilement entre les démontages/remontages (true par défaut).

### Concepts FastComments

Les concepts principaux à connaître pour démarrer sont `tenantId` et `urlId`. `tenantId` est l’identifiant de votre compte FastComments.com. `urlId` est l’endroit auquel les fils de commentaires seront rattachés. Cela peut être une URL de page, un identifiant de produit, un identifiant d’article, etc.

### Localisation

Tout le texte affiché aux utilisateurs dans ces widgets (étiquettes de boutons, espaces réservés, états vides, dates relatives comme « 5 minutes ago », messages d’erreur, etc.) est **piloté par le serveur**. Les composants ne codent pas en dur les chaînes anglaises ; ils affichent les traductions que FastComments fournit pour la locale demandée.

Pour demander une locale, définissez `locale` dans votre configuration :

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Lorsque aucune `locale` n’est définie, FastComments sert la langue par défaut du locataire.

**Modification du texte :** les traductions sont gérées dans votre tableau de bord FastComments, pas dans ce SDK. Pour changer la formulation, remplacez le texte par défaut ou ajoutez une langue, modifiez les traductions de votre compte dans le tableau de bord — la modification est prise en compte automatiquement par les widgets sans nécessiter de nouvelle version de l’application. Le SDK ne fournit aucun fallback anglais, ainsi toute clé que vous laissez vide dans le tableau de bord rendra un champ vide ; gardez les clés renseignées pour chaque langue que vous supportez.

### Notifications utilisateur

FastComments prend en charge les notifications pour [de nombreux scénarios](https://docs.fastcomments.com/guide-notifications.html). Les notifications sont configurables, peuvent être désactivées globalement ou au niveau d’une notification/commentaire, et supportent les abonnements au niveau de la page afin que les utilisateurs puissent s’abonner aux fils d’une page ou d’un article spécifique.

Par exemple, il est possible d’utiliser le SSO sécurisé pour authentifier l’utilisateur puis d’interroger périodiquement les notifications non lues et de les pousser vers l’utilisateur.

Voir [l’exemple AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) pour savoir comment récupérer et traduire les notifications utilisateur non lues.

### Navigateur GIF

Par défaut, aucune sélection d’image ou de GIF n’est activée. Consultez [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) pour savoir comment prendre en charge les téléchargements d’images et de GIF. Il existe un navigateur GIF qui anonymise les recherches et les images fournis dans cette bibliothèque, il vous suffit de l’utiliser.

### Performance

Veuillez ouvrir un ticket avec un exemple reproductible, incluant l’appareil utilisé, si vous identifiez un problème de performance. La performance est un citoyen de première classe de toutes les bibliothèques FastComments.