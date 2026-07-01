Le SDK FastComments Rust se compose de plusieurs modules :

- **Client Module** – Client API pour les API REST de FastComments  
  - Définitions de type complètes pour tous les modèles d'API  
  - Trois clients API couvrant toutes les méthodes FastComments :  
    - `default_api` (**DefaultApi**) – méthodes authentifiées par clé d'API pour une utilisation côté serveur  
    - `public_api` (**PublicApi**) – méthodes publiques, sans clé d'API, sécuritaires à appeler depuis les navigateurs et les applications mobiles  
    - `moderation_api` (**ModerationApi**) – une suite étendue d'API de modération en direct et rapides. Chaque méthode de Modération accepte un paramètre `sso` et peut s'authentifier via SSO ou un cookie de session FastComments.com.  
  - Prise en charge complète de async/await avec tokio  
  - Voir [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) pour la documentation détaillée de l'API  

- **SSO Module** – Utilitaires de Single Sign-On côté serveur  
  - Génération sécurisée de jetons pour l'authentification des utilisateurs  
  - Prise en charge des modes SSO simple et sécurisé  
  - Signature de jetons basée sur HMAC‑SHA256  

- **Core Types** – Définitions de type partagées et utilitaires  
  - Modèles de commentaires et structures de métadonnées  
  - Configurations d'utilisateur et de locataire  
  - Fonctions d'assistance pour les opérations courantes