Le SDK Swift de FastComments se compose de plusieurs modules :

- **Module Client** - Client API généré automatiquement pour les API REST de FastComments
  - Définitions complètes des types pour tous les modèles d'API
  - Points de terminaison authentifiés (`DefaultAPI`) et publics (`PublicAPI`)
  - Prise en charge complète d'async/await
  - Voir [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) pour la documentation détaillée de l'API

- **Module SSO** - Utilitaires côté serveur pour le Single Sign-On
  - Génération sécurisée de jetons pour l'authentification des utilisateurs
  - Prise en charge des modes SSO simple et sécurisé
  - Signature de jetons basée sur HMAC-SHA256 utilisant CryptoKit