La page des paramètres du plugin se trouve dans **Administration du site > Plugins > Plugins locaux > FastComments**. Les options disponibles sont :

#### Tenant ID

Votre Tenant ID FastComments. Trouvez-le dans le <a href="https://fastcomments.com/auth/my-account" target="_blank">tableau de bord FastComments</a> dans les paramètres de votre compte.

#### API Secret

Votre clé API Secret, requise pour le mode SSO sécurisé. Trouvez-la à <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Mon compte > API Secret</a>.

#### SSO Mode

Choisissez comment les utilisateurs sont authentifiés. Voir la section [SSO Modes](#moodle-sso-modes) pour des détails sur chaque option.

- **Secure** (recommandé) - authentification signée HMAC-SHA256 côté serveur
- **Simple** - données utilisateur côté client sans signature
- **None** - commentaires anonymes, pas d'intégration de connexion Moodle

#### Page Contexts

Contrôle où les commentaires apparaissent :

- **Course pages** - commentaires sur les pages principales du cours
- **Module/activity pages** - commentaires sur les activités et ressources individuelles
- **Both** - commentaires sur tous les types de pages

#### Commenting Style

Choisissez l'expérience de commentaire. Voir [Commenting Styles](#moodle-commenting-styles) pour des captures d'écran de chaque mode.

- **Comments** - widget de commentaires filaires standard sous le contenu de la page
- **Collab Chat** - discussions par sélection de texte en ligne avec indicateurs de présence
- **Both** - commentaires et Collab Chat actifs simultanément

#### CDN URL

L'URL CDN de FastComments. Par défaut `https://cdn.fastcomments.com`. Changez ceci pour l'URL CDN UE si vos données sont hébergées dans la région UE.