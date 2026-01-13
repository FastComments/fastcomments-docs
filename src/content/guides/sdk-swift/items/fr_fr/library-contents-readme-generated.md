Le SDK Swift de FastComments se compose de plusieurs modules :

- **Client Module** - Client d'API généré automatiquement pour les API REST de FastComments
  - Définitions complètes des types pour tous les modèles d'API
  - Points de terminaison à la fois authentifiés (`DefaultAPI`) et publics (`PublicAPI`)
  - Prise en charge complète d'async/await
  - Voir [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) pour la documentation détaillée de l'API

- **SSO Module** - Utilitaires Single Sign-On côté serveur
  - Génération sécurisée de jetons pour l'authentification des utilisateurs
  - Prise en charge à la fois des modes SSO simple et sécurisé
  - Signature de jetons basée sur HMAC-SHA256 utilisant CryptoKit