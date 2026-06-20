Le SDK FastComments Swift se compose de plusieurs modules :

- **Module Client** - client API pour les API REST de FastComments
  - Définitions de types complètes pour tous les modèles d'API
  - Méthodes authentifiées (`DefaultAPI`), publiques (`PublicAPI`) et de modération (`ModerationAPI`)
  - Prise en charge complète d'async/await
  - Consultez [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) pour la documentation détaillée de l'API

- **Module SSO** - utilitaires côté serveur pour Single Sign-On
  - Génération sécurisée de jetons pour l'authentification des utilisateurs
  - Prise en charge des modes SSO simple et sécurisé
  - Signature des jetons basée sur HMAC-SHA256 avec CryptoKit