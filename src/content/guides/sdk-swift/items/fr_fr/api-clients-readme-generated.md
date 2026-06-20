Le SDK FastComments fournit trois clients d'API :

### PublicAPI - Méthodes sûres côté client

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- Ne nécessitent pas de clé API
- Peuvent utiliser des jetons SSO pour l'authentification
- Sont soumis à une limitation de débit par utilisateur/appareil
- Conviennent aux applications destinées aux utilisateurs finaux

**Exemple d'utilisation**: Récupération et création de commentaires dans votre application iOS

### DefaultAPI - Méthodes côté serveur

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- Nécessitent votre clé API FastComments
- Doivent UNIQUEMENT être appelées depuis du code côté serveur
- Offrent un accès complet à vos données FastComments
- Sont soumis à une limitation de débit par locataire

**Exemple d'utilisation**: Opérations administratives, exportation de données en masse, gestion des utilisateurs

### ModerationAPI - Méthodes du tableau de bord des modérateurs

The `ModerationAPI` contains methods that power the moderator dashboard. These methods cover:
- **Modération des commentaires** - lister, compter, rechercher, récupérer les journaux, et exporter les commentaires
- **Actions de modération** - supprimer/restaurer des commentaires, signaler, définir l'état de revue/spam/approbation, gérer les votes, et rouvrir/fermer des fils de discussion
- **Bannissements** - bannir un utilisateur d'un commentaire, annuler des bannissements, récupérer des résumés pré-bannissement, vérifier le statut et les préférences de bannissement, et lire les comptes d'utilisateurs bannis
- **Badges et confiance** - attribuer/retirer des badges, lister les badges manuels, obtenir/définir le facteur de confiance d'un utilisateur, et lire le profil interne d'un utilisateur

Every `ModerationAPI` method accepts an `sso` parameter so moderators can be authenticated via SSO.

**Exemple d'utilisation**: Créer une expérience de modération pour les modérateurs de votre communauté

**IMPORTANT** : N'exposez jamais votre clé API dans du code côté client. Les clés API doivent être utilisées uniquement côté serveur.