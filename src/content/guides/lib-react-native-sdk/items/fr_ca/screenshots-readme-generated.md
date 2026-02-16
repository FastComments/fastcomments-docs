#### Thème : Erebus
![Thème : Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Thème : Par défaut
![Thème : Par défaut](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Éditeur WYSIWYG natif avec prise en charge des images !
![Éditeur WYSIWYG natif avec prise en charge des images](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Éditeur de texte enrichi

Cette bibliothèque utilise l'éditeur 10tap pour les fonctionnalités d'édition de texte enrichi, qui offre une expérience WYSIWYG puissante.

### Options de configuration

Cette bibliothèque vise à prendre en charge toutes les options de configuration définies dans [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tout comme l'implémentation web.

### Concepts FastComments

Les principaux concepts à connaître pour commencer sont `tenantId` et `urlId`. `tenantId` est l'identifiant de votre compte FastComments.com. `urlId` correspond à l'entité à laquelle les fils de commentaires seront liés. Il peut s'agir d'une URL de page, d'un identifiant de produit, d'un identifiant d'article, etc.

### Notifications utilisateur

FastComments prend en charge les notifications pour [de nombreux scénarios](https://docs.fastcomments.com/guide-notifications.html). Les notifications sont configurables, peuvent être désactivées globalement ou au niveau d'une notification/commentaire, et prennent en charge les abonnements au niveau de la page afin que les utilisateurs puissent s'abonner aux fils d'une page ou d'un article spécifique.

Par exemple, il est possible d'utiliser Secure SSO pour authentifier l'utilisateur, puis d'interroger périodiquement les notifications non lues et de les pousser vers l'utilisateur.

Consultez [l'exemple AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) pour savoir comment obtenir et traduire les notifications utilisateur non lues.

### Navigateur GIF

Par défaut, aucune sélection d'image ou de GIF n'est activée. Consultez [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) pour savoir comment prendre en charge les téléchargements d'images et de GIF. Il existe un Navigateur GIF qui anonymise les recherches et les images fournies dans cette bibliothèque; il vous suffit de l'utiliser.

### Performances

Veuillez ouvrir un ticket avec un exemple permettant de reproduire le problème, en précisant l'appareil utilisé, si vous identifiez des problèmes de performance. Les performances sont une priorité dans toutes les bibliothèques FastComments.