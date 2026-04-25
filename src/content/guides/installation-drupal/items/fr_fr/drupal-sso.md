FastComments s'intègre au système d'utilisateurs de Drupal via SSO, ou authentification unique. Vos utilisateurs se connectent à votre site Drupal, et le module transmet automatiquement leur identité à FastComments. Aucun compte supplémentaire à créer, aucune synchronisation initiale à effectuer.

Le module prend en charge trois modes SSO, configurés sous `Administration > Configuration > Content > FastComments`.

### Aucun

Pas de SSO. Les utilisateurs commentent en tant qu'invités ou créent un compte FastComments. Utilisez ceci si votre site est public et que vous n'avez pas besoin d'associer les commentaires aux utilisateurs Drupal.

### Simple

Transmet le nom, l'email et l'avatar de l'utilisateur Drupal à FastComments sans vérification côté serveur. Aucun API Secret n'est nécessaire. Adapté aux sites internes ou à faible risque.

### Sécurisé (recommandé)

Utilise [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) pour vérifier chaque identité utilisateur avec FastComments. C'est le mode recommandé lorsque vous avez configuré un API Secret, et c'est le seul mode qui empêche un visiteur d'usurper l'identité d'un autre utilisateur.

L'identité de l'utilisateur est transmise à FastComments chaque fois qu'un utilisateur consulte un fil de commentaires. Il n'y a pas de synchronisation initiale ou continue à exécuter.

<sup>(Optionnel)</sup> Ajoutez vos administrateurs à [Utilisateurs & Administrateurs](https://fastcomments.com/auth/my-account/users) et vos modérateurs à [Modérateurs de commentaires](https://fastcomments.com/auth/my-account/moderate-comments/moderators) pour améliorer leur expérience et activer le suivi des statistiques pour les modérateurs.

Pour une analyse plus approfondie du fonctionnement du SSO, consultez la [section SSO](/guide-customizations-and-configuration.html#sso) de la documentation sur les personnalisations.

---