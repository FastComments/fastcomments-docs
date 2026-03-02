La page des paramètres du plugin se trouve dans **Administration du site > Plugins > Plugins locaux > FastComments**. Les options disponibles sont :

#### ID de locataire

Votre ID de locataire FastComments. Trouvez-le dans le <a href="https://fastcomments.com/auth/my-account" target="_blank">tableau de bord FastComments</a> dans les paramètres de votre compte.

#### Clé secrète de l'API

Votre clé secrète API, requise pour le mode SSO sécurisé. Trouvez-la sur <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Mon compte > Secret API</a>.

#### Mode SSO

Choisissez comment les utilisateurs sont authentifiés. Voir la section [Modes SSO](#moodle-sso-modes) pour les détails sur chaque option.

- **Sécurisé** (recommandé) - authentification signée côté serveur HMAC-SHA256
- **Simple** - données utilisateur côté client sans signature
- **Aucun** - commentaires anonymes, pas d'intégration avec la connexion Moodle

#### Contextes de page

Contrôle où les commentaires apparaissent :

- **Pages du cours** - commentaires sur les pages principales du cours
- **Pages de module/activité** - commentaires sur les activités et ressources individuelles
- **Les deux** - commentaires sur tous les types de pages

#### Style de commentaire

Choisissez l'expérience de commentaires. Voir [Styles de commentaires](#moodle-commenting-styles) pour des captures d'écran de chaque mode.

- **Commentaires** - widget de commentaires en fil standard sous le contenu de la page
- **Collab Chat** - discussions inline par sélection de texte avec indicateurs de présence
- **Les deux** - commentaires et Collab Chat actifs simultanément

#### URL du CDN

L'URL du CDN FastComments. Valeur par défaut : `https://cdn.fastcomments.com`. Modifiez ceci pour l'URL du CDN UE si vos données sont hébergées dans la région UE.