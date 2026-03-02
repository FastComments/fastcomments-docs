La page des paramètres du plugin se trouve dans **Administration du site > Plugins > Plugins locaux > FastComments**. Les options disponibles sont :

#### ID du locataire

Votre ID de locataire FastComments. Trouvez-le dans le <a href="https://fastcomments.com/auth/my-account" target="_blank">tableau de bord FastComments</a> dans les paramètres de votre compte.

#### Clé secrète de l'API

Votre clé API secrète, requise pour le mode SSO sécurisé. Trouvez-la à <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Mon compte > Secret API</a>.

#### Mode SSO

Choisissez comment les utilisateurs sont authentifiés. Voir la section [SSO Modes](#items-moodle-sso-modes) pour des détails sur chaque option.

- **Secure** (recommandé) - authentification signée HMAC-SHA256 côté serveur
- **Simple** - données utilisateur côté client sans signature
- **None** - commentaires anonymes, pas d'intégration de connexion Moodle

#### Contexte des pages

Contrôle où les commentaires apparaissent :

- **Pages de cours** - commentaires sur les pages principales des cours
- **Pages des modules/activités** - commentaires sur les activités et ressources individuelles
- **Les deux** - commentaires sur tous les types de pages

#### Style de commentaires

Choisissez l'expérience de commentaires. Voir [Styles de commentaires](#items-moodle-commenting-styles) pour des captures d'écran de chaque mode.

- **Comments** - widget de commentaires filaire standard sous le contenu de la page
- **Collab Chat** - discussions en ligne via sélection de texte avec indicateurs de présence
- **Both** - commentaires et collab chat actifs simultanément

#### URL du CDN

L'URL du CDN FastComments. Par défaut : `https://cdn.fastcomments.com`. Changez ceci pour l'URL du CDN UE si vos données sont hébergées dans la région UE.

---