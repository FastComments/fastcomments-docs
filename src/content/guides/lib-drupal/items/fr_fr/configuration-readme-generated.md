Accédez à **Administration > Configuration > Contenu > FastComments** (`/admin/config/content/fastcomments`).

### Paramètres

- **ID du locataire** (requis) - Votre ID de locataire FastComments. Retrouvez-le dans [Paramètres > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **Secret API** - Requis pour le SSO sécurisé, la vérification des webhooks et la synchronisation des pages. Trouvé sous [Paramètres > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **Mode SSO** - Intégration Single Sign-On :
  - **Aucun** - Pas de SSO, les utilisateurs commentent en tant qu'invités ou créent des comptes FastComments.
  - **Simple** - Transmet les informations de l'utilisateur Drupal (nom, e-mail, avatar) à FastComments sans vérification côté serveur.
  - **Sécurisé** - Utilise une vérification HMAC-SHA256 pour authentifier de manière sécurisée les utilisateurs Drupal auprès de FastComments (recommandé).
- **Style de commentaires** - Le type de widget à afficher :
  - **Commentaires en direct** - Commentaires filaires en temps réel.
  - **Chat en streaming** - Interface de chat en direct.
  - **Chat collaboratif** - Annotation collaborative par sélection de texte sur la zone de contenu principal.
  - **Chat collaboratif + commentaires** - À la fois chat collaboratif et commentaires standard.
- **URL CDN** - URL du CDN FastComments (par défaut : `https://cdn.fastcomments.com`).
- **URL du site** - URL du site FastComments (par défaut : `https://fastcomments.com`).
- **Notifications par e-mail** - Envoyer un e-mail aux auteurs du contenu lorsqu'un nouveau commentaire est publié sur leur contenu.

### Ajout de commentaires aux types de contenu

Ajoutez le champ **FastComments** à vos types de contenu via **Structure > Types de contenu > [type] > Gérer les champs**. Le champ dispose d'un interrupteur d'état et d'un identifiant personnalisé optionnel par entité.

### Résidence des données dans l'UE

Pour la résidence des données dans l'UE, mettez à jour :
- **URL CDN** vers `https://cdn-eu.fastcomments.com`
- **URL du site** vers `https://eu.fastcomments.com`