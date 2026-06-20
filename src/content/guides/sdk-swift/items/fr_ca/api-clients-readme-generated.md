Le SDK FastComments fournit trois clients d'API :

### PublicAPI - Méthodes sûres côté client

Le `PublicAPI` contient des méthodes qui sont sûres à appeler depuis du code côté client (applications iOS/macOS). Ces méthodes :
- Ne nécessitent pas de clé API
- Peuvent utiliser des jetons SSO pour l'authentification
- Sont soumises à des limites de débit par utilisateur/appareil
- Conviennent aux applications destinées aux utilisateurs finaux

**Exemple d'utilisation** : Récupérer et créer des commentaires dans votre application iOS

### DefaultAPI - Méthodes côté serveur

Le `DefaultAPI` contient des méthodes authentifiées qui nécessitent une clé API. Ces méthodes :
- Exigent votre clé API FastComments
- Doivent UNIQUEMENT être appelées depuis du code côté serveur
- Offrent un accès complet à vos données FastComments
- Sont soumises à des limites de débit par tenant

**Exemple d'utilisation** : Opérations administratives, exportation de données en masse, gestion des utilisateurs

### ModerationAPI - Méthodes pour le tableau de bord des modérateurs

Le `ModerationAPI` contient des méthodes qui alimentent le tableau de bord des modérateurs. Ces méthodes couvrent :
- **Modération des commentaires** - lister, compter, rechercher, récupérer les journaux et exporter les commentaires
- **Actions de modération** - supprimer/restaurer des commentaires, signaler, définir le statut de révision/spam/approbation, gérer les votes, et rouvrir/fermer des fils de discussion
- **Bannissements** - bannir un utilisateur d'un commentaire, annuler des bannissements, récupérer des résumés avant bannissement, vérifier le statut et les préférences de bannissement, et lire le nombre d'utilisateurs bannis
- **Badges et confiance** - attribuer/retirer des badges, lister les badges manuels, obtenir/définir le facteur de confiance d'un utilisateur, et lire le profil interne d'un utilisateur

Chaque méthode `ModerationAPI` accepte un paramètre `sso` afin que les modérateurs puissent être authentifiés via SSO.

**Exemple d'utilisation** : Créer une expérience de modération pour les modérateurs de votre communauté

**IMPORTANT** : Ne divulguez jamais votre clé API dans du code côté client. Les clés API doivent être utilisées uniquement côté serveur.