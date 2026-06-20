Le FastComments Rust SDK se compose de plusieurs modules :

- **Client Module** - Client d'API pour les API REST de FastComments
  - Définitions de types complètes pour tous les modèles d'API
  - Trois clients d'API couvrant toutes les méthodes de FastComments :
    - `default_api` (**DefaultApi**) - méthodes authentifiées par clé API pour usage côté serveur
    - `public_api` (**PublicApi**) - méthodes publiques, sans clé API, sûres à appeler depuis les navigateurs et les applications mobiles
    - `moderation_api` (**ModerationApi**) - méthodes supportant le tableau de bord de modération, y compris la modération des commentaires (liste, comptage, recherche, journaux, export), actions de modération (suppression/restauration, signalement, définition du statut de révision/spam/approbation, votes, réouverture/fermeture de fil), bannissements (bannir à partir d'un commentaire, annuler, résumés de pré-bannissement, statut/préférences de bannissement, comptages d'utilisateurs bannis), et badges & confiance (attribuer/retirer des badges, badges manuels, obtenir/définir le facteur de confiance, profil interne utilisateur). Chaque méthode de Moderation accepte un paramètre `sso` afin que l'appel puisse être effectué au nom d'un modérateur authentifié via SSO.
  - Prise en charge complète d'async/await avec tokio
  - Voir [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) pour une documentation détaillée de l'API

- **SSO Module** - Utilitaires Single Sign-On côté serveur
  - Génération sécurisée de jetons pour l'authentification des utilisateurs
  - Prise en charge des modes SSO simple et sécurisé
  - Signature de jetons basée sur HMAC-SHA256

- **Core Types** - Définitions de types partagées et utilitaires
  - Modèles de commentaire et structures de métadonnées
  - Configurations d'utilisateur et de locataire
  - Fonctions utilitaires pour les opérations courantes