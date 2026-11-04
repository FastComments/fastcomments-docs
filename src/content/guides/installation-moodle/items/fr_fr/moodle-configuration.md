La page des paramètres du plugin se trouve dans **Site Administration > Plugins > Local plugins > FastComments**. Les options disponibles sont :

#### Tenant ID

Votre Tenant ID FastComments. Vous le trouverez dans le <a href="https://fastcomments.com/auth/my-account" target="_blank">tableau de bord FastComments</a> sous les paramètres de votre compte.

#### API Secret

Votre clé API Secret, requise pour le mode Secure SSO. Vous la trouverez dans <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Choisissez comment les utilisateurs sont authentifiés. Consultez la section [SSO Modes](#moodle-sso-modes) pour plus de détails sur chaque option.

- **Secure** (recommandé) - authentification signée côté serveur avec HMAC-SHA256
- **Simple** - données utilisateur côté client sans signature
- **None** - commentaires anonymes, sans intégration de connexion Moodle

#### Page Contexts

Contrôle l'emplacement d'affichage des commentaires :

- **Course pages** - commentaires sur les pages principales des cours
- **Module/activity pages** - commentaires sur les activités et ressources individuelles
- **Both** - commentaires sur tous les types de pages

#### Commenting Style

Choisissez l'expérience de commentaire. Consultez [Commenting Styles](#moodle-commenting-styles) pour des captures d'écran de chaque mode.

- **Comments** - widget de commentaires standard avec fils de discussion sous le contenu de la page
- **Collab Chat** - discussions sur la sélection de texte en ligne avec indicateurs de présence
- **Both** - commentaires et chat collaboratif actifs ensemble

#### CDN URL

L'URL du CDN FastComments. Par défaut : `https://cdn.fastcomments.com`. Modifiez-la pour l'URL du CDN UE si vos données sont hébergées dans la région UE.