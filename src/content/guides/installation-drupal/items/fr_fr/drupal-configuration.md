Tous les paramètres se trouvent sous `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Obligatoire

- **Tenant ID** - Votre identifiant de locataire FastComments. Trouvez‑le sous [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Requis pour le SSO sécurisé, la vérification des webhooks et la synchronisation de page. Trouvé sous [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Stockage du secret API

Le module lit le secret API à partir de la première de ces sources qui possède une valeur :

1. **Key module** - Installez [Key](https://www.drupal.org/project/key), créez une clé *Authentication*, et choisissez‑la sous **API Secret key** dans le formulaire de paramètres. La clé peut être soutenue par une variable d'environnement ou un fichier en dehors de la racine web.
2. **settings.php** - Ajoutez `$config['fastcomments.settings']['api_secret'] = 'your-secret';` à `settings.php` ou `settings.local.php`. Le secret n'entre jamais dans la configuration exportée.
3. **The settings form** - Saisissez‑le dans le champ **API Secret**. Cela stocke la valeur dans `fastcomments.settings`, de sorte qu'elle se retrouve dans les exportations de configuration. Le rapport d'état de Drupal avertit lorsque c’est la source active.

Le formulaire indique quelle source est active. Laisser le champ vide conserve le secret stocké. Cochez **Remove the API Secret stored in configuration** pour le supprimer après avoir déplacé le secret vers le module Key ou `settings.php`.

## Style de commentaires

Choisissez le widget qui correspond à la façon dont vous souhaitez que les visiteurs s’expriment sur votre site.

- **Live Comments** - Commentaires en fil de discussion en temps réel.
- **Streaming Chat** - Interface de chat en direct, idéale pour les événements et les diffusions en direct.
- **Collab Chat** - Annotation de sélection de texte sur la zone de contenu principal. Les visiteurs mettent en surbrillance le texte et démarrent une discussion dans le contexte.
- **Collab Chat + Comments** - À la fois le collab chat et les commentaires standards sur la même page.

## Mode SSO

- **None** - Aucun SSO. Les utilisateurs commentent en tant qu’invités ou créent un compte FastComments.
- **Simple** - Transmet les informations utilisateur Drupal (nom, e‑mail, avatar) à FastComments sans vérification côté serveur.
- **Secure** - Utilise HMAC‑SHA256 pour vérifier les utilisateurs Drupal avec FastComments. Recommandé lorsque vous avez configuré un secret API.

Voir la section `Single Sign-On (SSO)` pour plus de détails.

## Autres paramètres

- **CDN URL** - Par défaut `https://cdn.fastcomments.com`.
- **Site URL** - Par défaut `https://fastcomments.com`.
- **Email notifications** - Envoyer un e‑mail à l’auteur du contenu lorsqu’un nouveau commentaire est publié sur son contenu.

Pour la résidence des données UE, voir la section `EU Data Residency`.

---