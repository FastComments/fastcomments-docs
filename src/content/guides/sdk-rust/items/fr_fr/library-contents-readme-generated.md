Le SDK FastComments Rust comprend plusieurs modules :

- **Module Client** - client API pour les API REST FastComments
  - Définitions complètes des types pour tous les modèles d'API
  - Trois clients API couvrant toutes les méthodes FastComments :
    - `default_api` (**DefaultApi**) – méthodes authentifiées par clé API pour une utilisation côté serveur
    - `public_api` (**PublicApi**) – méthodes publiques, sans clé API, sécurisées pour être appelées depuis les navigateurs et les applications mobiles
    - `moderation_api` (**ModerationApi**) – une suite étendue d’API de modération en temps réel et rapide. Chaque méthode de modération accepte un paramètre `sso` et peut s’authentifier via SSO ou un cookie de session FastComments.com.
  - Support complet async/await avec tokio
  - Voir [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) pour une documentation détaillée de l'API

- **Module SSO** – utilitaires d’authentification unique côté serveur
  - Génération sécurisée de jetons pour l’authentification des utilisateurs
  - Prise en charge des modes SSO simple et sécurisé
  - Signature de jetons basée sur HMAC‑SHA256

- **Types de base** – définitions de types partagés et utilitaires
  - Modèles de commentaires et structures de métadonnées
  - Configurations d’utilisateur et de locataire
  - Fonctions d’assistance pour les opérations courantes