Le FastComments Rust SDK se compose de plusieurs modules :

- **Client Module** - Client d'API généré automatiquement pour les FastComments REST APIs
  - Définitions complètes des types pour tous les modèles d'API
  - Points de terminaison authentifiés (`DefaultApi`) et publics (`PublicApi`)
  - Prise en charge complète d'async/await avec tokio
  - Voir [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) pour la documentation détaillée de l'API

- **SSO Module** - Utilitaires Single Sign-On côté serveur
  - Génération sécurisée de jetons pour l'authentification des utilisateurs
  - Prise en charge des modes SSO simple et sécurisé
  - Signature de jetons basée sur HMAC-SHA256

- **Core Types** - Définitions de types et utilitaires partagés
  - Modèles de commentaires et structures de métadonnées
  - Configurations des utilisateurs et des locataires
  - Fonctions utilitaires pour les opérations courantes