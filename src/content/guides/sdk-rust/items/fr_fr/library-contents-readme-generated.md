Le SDK Rust de FastComments se compose de plusieurs modules :

- **Client Module** - Client API généré automatiquement pour les API REST de FastComments
  - Définitions de types complètes pour tous les modèles d'API
  - Points de terminaison à la fois authentifiés (`DefaultApi`) et publics (`PublicApi`)
  - Prise en charge complète d'async/await avec tokio
  - Voir [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) pour la documentation détaillée de l'API

- **SSO Module** - Utilitaires Single Sign-On côté serveur
  - Génération sécurisée de jetons pour l'authentification des utilisateurs
  - Prise en charge des modes SSO simple et sécurisé
  - Signature des jetons basée sur HMAC-SHA256

- **Core Types** - Définitions de types partagées et utilitaires
  - Modèles de commentaires et structures de métadonnées
  - Configurations des utilisateurs et des locataires
  - Fonctions utilitaires pour les opérations courantes