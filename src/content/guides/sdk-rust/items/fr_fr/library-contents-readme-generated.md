---
Le SDK Rust FastComments se compose de plusieurs modules :

- **Client Module** - Client API pour les API REST de FastComments
  - Définitions de types complètes pour tous les modèles d'API
  - Trois clients API couvrant toutes les méthodes FastComments :
    - `default_api` (**DefaultApi**) - méthodes authentifiées par clé API pour une utilisation côté serveur
    - `public_api` (**PublicApi**) - méthodes publiques sans clé API, sûres à appeler depuis des navigateurs et des applications mobiles
    - `moderation_api` (**ModerationApi**) - méthodes supportant le tableau de bord des modérateurs, y compris la modération des commentaires (liste, comptage, recherche, journaux, export), les actions de modération (suppression/restauration, signalement, définition du statut revue/spam/approbation, votes, réouverture/fermeture de fil), les bannissements (bannir à partir d'un commentaire, annuler, résumés avant bannissement, statut/préférences de bannissement, décomptes d'utilisateurs bannis), et badges & confiance (attribuer/supprimer des badges, badges manuels, obtenir/définir le facteur de confiance, profil interne de l'utilisateur). Chaque méthode de modération accepte un paramètre `sso` afin que l'appel puisse être effectué au nom d'un modérateur authentifié via SSO.
  - Prise en charge complète d'async/await avec tokio
  - Voir [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) pour une documentation détaillée de l'API

- **SSO Module** - Utilitaires Single Sign-On côté serveur
  - Génération sécurisée de jetons pour l'authentification des utilisateurs
  - Prise en charge des modes SSO simple et sécurisé
  - Signature de jetons basée sur HMAC-SHA256

- **Core Types** - Définitions de types partagées et utilitaires
  - Modèles de commentaires et structures de métadonnées
  - Configurations des utilisateurs et des locataires
  - Fonctions utilitaires pour les opérations courantes
---