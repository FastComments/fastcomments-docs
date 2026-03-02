La page des paramètres du plugin se trouve dans **Site Administration > Plugins > Local plugins > FastComments**. Les options disponibles sont :

#### Tenant ID

Votre FastComments Tenant ID. Trouvez-le dans le <a href="https://fastcomments.com/auth/my-account" target="_blank">tableau de bord FastComments</a> sous les paramètres de votre compte.

#### API Secret

Votre API Secret key, requise pour le mode SSO Secure. Trouvez-la à <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Mon compte > API Secret</a>.

#### SSO Mode

Choisissez comment les utilisateurs sont authentifiés. Voir la section [SSO Modes](#moodle-sso-modes) pour les détails sur chaque option.

- **Secure** (recommandé) - authentification signée côté serveur HMAC-SHA256
- **Simple** - données utilisateur côté client sans signature
- **None** - commentaires anonymes, pas d'intégration de connexion Moodle

#### Page Contexts

Contrôle où les commentaires apparaissent :

- **Course pages** - commentaires sur les pages principales du cours
- **Module/activity pages** - commentaires sur les activités et ressources individuelles
- **Both** - commentaires sur tous les types de pages

#### Commenting Style

Choisissez l'expérience de commentaires. Consultez [Commenting Styles](#moodle-commenting-styles) pour des captures d'écran de chaque mode.

- **Comments** - widget de commentaires filaire standard sous le contenu de la page
- **Collab Chat** - discussions en ligne basées sur la sélection de texte avec indicateurs de présence
- **Both** - les commentaires et Collab Chat actifs simultanément

#### CDN URL

L'URL CDN de FastComments. Par défaut : `https://cdn.fastcomments.com`. Modifiez ceci pour l'URL CDN de l'UE si vos données sont hébergées dans la région UE.