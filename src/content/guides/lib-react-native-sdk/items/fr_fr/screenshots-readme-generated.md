#### Thème : Erebus
![Thème : Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Thème : Default
![Thème : Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Éditeur WYSIWYG natif avec prise en charge des images !
![Éditeur WYSIWYG natif avec prise en charge des images](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Éditeur de texte enrichi

Cette bibliothèque utilise [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) pour l'édition de texte enrichi, qui fournit une puissante expérience d'édition WYSIWYG. Le même éditeur alimente iOS, Android et le web (via `react-native-web`), de sorte que le composeur se comporte de manière cohérente sur toutes les plateformes avec une seule implémentation.

`react-native-enriched` requiert la Nouvelle Architecture React Native (Fabric) côté natif, ainsi qu'un bundler qui résout les conditions d'`exports` du package (Metro avec package exports / RN 0.72+). Le support web est actuellement expérimental.

### Options de configuration

Cette bibliothèque vise à prendre en charge toutes les options de configuration définies dans [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tout comme l'implémentation web.

### Principaux concepts FastComments

Les principaux concepts à connaître pour commencer sont `tenantId` et `urlId`. `tenantId` est l'identifiant de votre compte FastComments.com. `urlId` correspond à l'endroit auquel les fils de commentaires seront rattachés. Cela peut être une URL de page, un identifiant de produit, un identifiant d'article, etc.

### Notifications utilisateur

FastComments prend en charge les notifications pour [de nombreux scénarios](https://docs.fastcomments.com/guide-notifications.html). Les notifications sont configurables, peuvent être désactivées globalement ou au niveau d'une notification/commentaire, et prennent en charge les abonnements au niveau de la page afin que les utilisateurs puissent s'abonner aux fils d'une page ou d'un article spécifique.

Par exemple, il est possible d'utiliser Secure SSO pour authentifier l'utilisateur puis de sonder périodiquement les notifications non lues et de les transmettre à l'utilisateur.

Voir [l'exemple AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) pour savoir comment récupérer et traduire les notifications utilisateur non lues.

### Navigateur GIF

Par défaut, aucune sélection d'image ou de GIF n'est activée. Voir [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) pour savoir comment prendre en charge les téléversements d'images et de GIF. Il existe un Navigateur GIF qui anonymise les recherches et les images fourni dans cette bibliothèque, il vous suffit de l'utiliser.

### Performance

Merci d'ouvrir un ticket avec un exemple permettant de reproduire le problème, en précisant l'appareil utilisé, si vous identifiez des problèmes de performance. Les performances sont une priorité de premier ordre pour toutes les bibliothèques FastComments.