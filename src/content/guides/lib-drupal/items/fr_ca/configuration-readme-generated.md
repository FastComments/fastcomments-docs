Accédez à **Administration > Configuration > Contenu > FastComments** (`/admin/config/content/fastcomments`).

### Paramètres

- **ID du locataire** (requis) - Votre Tenant ID FastComments. Trouvez-le sous [Paramètres > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Requis pour le SSO sécurisé, la vérification des webhooks et la synchronisation des pages. Trouvé sous [Paramètres > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **Mode SSO** - Intégration d'authentification unique :
  - **Aucun** - Pas de SSO, les utilisateurs commentent en tant qu'invités ou créent des comptes FastComments.
  - **Simple** - Transmet les infos utilisateur Drupal (nom, courriel, avatar) à FastComments sans vérification côté serveur.
  - **Sécurisé** - Utilise la vérification HMAC-SHA256 pour authentifier de façon sécurisée les utilisateurs Drupal auprès de FastComments (recommandé).
- **Style de commentaire** - Le type de widget à afficher :
  - **Commentaires en direct** - Commentaires en fil en temps réel.
  - **Chat en streaming** - Interface de chat en direct.
  - **Chat collaboratif** - Annotation collaborative par sélection de texte sur la zone de contenu principale.
  - **Chat collaboratif + commentaires** - À la fois chat collaboratif et commentaires standards.
- **URL CDN** - URL du CDN FastComments (par défaut : `https://cdn.fastcomments.com`).
- **URL du site** - URL du site FastComments (par défaut : `https://fastcomments.com`).
- **Notifications par courriel** - Envoyer un courriel aux auteurs du contenu lorsqu’un nouveau commentaire est publié sur leur contenu.

### Ajout de commentaires aux types de contenu

Ajoutez le champ **FastComments** à vos types de contenu via **Structure > Types de contenu > [type] > Gérer les champs**. Le champ possède un commutateur d'état et un identifiant personnalisé optionnel par entité.

### Résidence des données (UE)

Pour la résidence des données dans l'UE, mettez à jour :
- **URL CDN** vers `https://cdn-eu.fastcomments.com`
- **URL du site** vers `https://eu.fastcomments.com`