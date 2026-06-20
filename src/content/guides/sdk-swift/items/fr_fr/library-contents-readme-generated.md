---
Le SDK Swift FastComments se compose de plusieurs modules :

- **Module Client** - Client API pour les API REST de FastComments
  - Définitions complètes des types pour tous les modèles d'API
  - Méthodes authentifiées (`DefaultAPI`), publiques (`PublicAPI`) et de modération (`ModerationAPI`)
  - Prise en charge complète d'async/await
  - Voir [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) pour une documentation API détaillée

- **Module SSO** - Utilitaires Single Sign-On côté serveur
  - Génération sécurisée de jetons pour l'authentification des utilisateurs
  - Prise en charge des modes SSO simple et sécurisé
  - Signature des jetons basée sur HMAC-SHA256 en utilisant CryptoKit
---