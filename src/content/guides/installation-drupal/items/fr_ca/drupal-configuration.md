Tous les paramètres se trouvent sous `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Requis

- **Tenant ID** - Votre Tenant ID FastComments. Trouvez-le sous [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Requis pour le SSO sécurisé, la vérification des webhooks et la synchronisation des pages. Trouvé sous [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Style de commentaires

Choisissez le widget qui correspond à la manière dont vous voulez que les gens échangent sur votre site.

- **Live Comments** - Commentaires en temps réel, structurés en fils.
- **Streaming Chat** - Interface de chat en direct, idéale pour les événements et les diffusions en direct.
- **Collab Chat** - Annotation par sélection de texte sur la zone de contenu principale. Les visiteurs surlignent du texte et démarrent une discussion dans le contexte.
- **Collab Chat + Comments** - À la fois collab chat et commentaires standard sur la même page.

## Mode SSO

- **None** - Aucun SSO. Les utilisateurs commentent en tant qu'invités ou créent un compte FastComments.
- **Simple** - Transmet les informations utilisateur Drupal (nom, courriel, avatar) à FastComments sans vérification côté serveur.
- **Secure** - Utilise HMAC-SHA256 pour vérifier les utilisateurs Drupal avec FastComments. Recommandé si vous avez configuré un API Secret.

Voir la section `Single Sign-On (SSO)` pour les détails.

## Autres paramètres

- **CDN URL** - Par défaut : `https://cdn.fastcomments.com`.
- **Site URL** - Par défaut : `https://fastcomments.com`.
- **Notifications par courriel** - Envoyer un courriel à l'auteur du contenu lorsqu'un nouveau commentaire est publié sur son contenu.

Pour la résidence des données dans l'UE, voir la section `EU Data Residency`.