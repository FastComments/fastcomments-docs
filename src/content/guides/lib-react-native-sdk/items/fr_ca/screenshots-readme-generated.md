#### Thème : Erebus
![Thème : Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Thème : Par défaut
![Thème : Par défaut](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Éditeur WYSIWYG natif avec prise en charge des images !
![Éditeur WYSIWYG natif avec prise en charge des images](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Éditeur de texte enrichi

Cette bibliothèque utilise [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) pour l'édition de texte enrichi, ce qui offre une expérience WYSIWYG puissante. Le même éditeur alimente iOS, Android et le web (via `react-native-web`), de sorte que le composeur se comporte de manière cohérente sur toutes les plateformes avec une seule implémentation.

`react-native-enriched` nécessite la nouvelle architecture de React Native (Fabric) sur le natif, et un bundler qui résout les conditions d'exports des paquets (Metro avec package exports / RN 0.72+). La prise en charge du Web est actuellement expérimentale.

### Options de configuration

Cette bibliothèque vise à prendre en charge toutes les options de configuration définies dans [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tout comme l'implémentation web.

### Concepts FastComments

Les principaux concepts à connaître pour commencer sont `tenantId` et `urlId`. `tenantId` est l'identifiant de votre compte FastComments.com. `urlId` correspond à l'endroit auquel les fils de commentaires seront rattachés. Cela peut être une URL de page, un identifiant de produit, un identifiant d'article, etc.

### Notifications utilisateur

FastComments prend en charge les notifications pour [de nombreux scénarios](https://docs.fastcomments.com/guide-notifications.html). Les notifications sont configurables, peuvent être désactivées globalement ou au niveau d'une notification/commentaire, et prennent en charge les abonnements par page afin que les utilisateurs puissent s'abonner aux fils d'une page ou d'un article spécifique.

Par exemple, il est possible d'utiliser Secure SSO pour authentifier l'utilisateur puis de sonder périodiquement les notifications non lues et de les pousser vers l'utilisateur.

Voir [l'exemple AppNotificationsSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) pour savoir comment obtenir et traduire les notifications utilisateur non lues.

### Navigateur de GIF

Par défaut, aucune sélection d'images ou de GIF n'est activée. Voir [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) pour savoir comment prendre en charge les téléchargements d'images et de GIF. Il existe un Navigateur de GIF qui anonymise les recherches et les images fournies dans cette bibliothèque, il suffit de l'utiliser.

### Performance

Veuillez ouvrir un ticket avec un exemple permettant de reproduire le problème, en précisant l'appareil utilisé, si vous constatez des problèmes de performance. La performance est une priorité de premier plan dans toutes les bibliothèques FastComments.