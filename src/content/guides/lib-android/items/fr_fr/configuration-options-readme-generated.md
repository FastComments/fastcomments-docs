Le SDK fournit de nombreuses options de configuration via la classe `CommentWidgetConfig` :

| Option | Description |
|--------|-------------|
| `tenantId` | L'identifiant de votre compte FastComments |
| `urlId` | Identifiant représentant la page actuelle |
| `sso` | Jeton SSO pour l'authentification |
| `allowAnon` | Autoriser les commentaires anonymes |
| `voteStyle` | Style de vote UpDown ou Heart |
| `hideAvatars` | Masquer les avatars des utilisateurs |
| `hasDarkBackground` | Indique le mode sombre |
| `customCSS` | Styles CSS personnalisés |
| `enableInfiniteScrolling` | Activer la pagination par défilement infini |
| `readonly` | Désactiver les commentaires mais afficher les commentaires existants |
| `disableVoting` | Désactiver la fonctionnalité de vote |
| `disableLiveCommenting` | Désactiver les mises à jour en temps réel |
| `pollCreation` | Qui peut joindre un sondage à un commentaire : `NUMBER_0` désactivé, `NUMBER_1` administrateurs et modérateurs, `NUMBER_2` tout le monde |
| `pollVoting` | Qui peut voter sur les sondages : `NUMBER_0` utilisateurs connectés, `NUMBER_1` tout le monde |