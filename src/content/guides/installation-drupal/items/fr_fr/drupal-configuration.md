Tous les paramètres se trouvent sous `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Requis

- **Tenant ID** - Votre Tenant ID FastComments. Trouvez-le sous [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Nécessaire pour le SSO sécurisé, la vérification des webhooks et la synchronisation des pages. Trouvé sous [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Style de commentaires

Choisissez le widget qui correspond à la façon dont vous voulez que les gens discutent sur votre site.

- **Live Comments** - Commentaires filaires en temps réel.
- **Streaming Chat** - Interface de chat en direct, idéale pour les événements et les livestreams.
- **Collab Chat** - Annotation par sélection de texte sur la zone de contenu principale. Les visiteurs surlignent le texte et démarrent une discussion dans son contexte.
- **Collab Chat + Comments** - À la fois collab chat et commentaires standard sur la même page.

## Mode SSO

- **None** - Pas de SSO. Les utilisateurs commentent en tant qu’invités ou créent un compte FastComments.
- **Simple** - Transmet les infos utilisateur Drupal (nom, email, avatar) à FastComments sans vérification côté serveur.
- **Secure** - Utilise HMAC-SHA256 pour vérifier les utilisateurs Drupal avec FastComments. Recommandé lorsque vous avez configuré un API Secret.

Voir la section `Single Sign-On (SSO)` pour les détails.

## Autres paramètres

- **CDN URL** - Par défaut `https://cdn.fastcomments.com`.
- **Site URL** - Par défaut `https://fastcomments.com`.
- **Email notifications** - Envoyer un email à l’auteur du contenu lorsqu’un nouveau commentaire est publié sur son contenu.

Pour la résidence des données dans l’UE, voir la section `EU Data Residency`.