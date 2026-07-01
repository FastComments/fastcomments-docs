Le SDK FastComments fournit trois clients API :

### PublicAPI - Méthodes sûres pour le client

Le `PublicAPI` contient des méthodes qui peuvent être appelées depuis le code côté client (applications iOS/macOS). Ces méthodes :
- Ne nécessitent pas de clé API
- Peuvent utiliser des jetons SSO pour l’authentification
- Sont limitées en débit par utilisateur/appareil
- Conviennent aux applications destinées aux utilisateurs finaux

**Exemple d’utilisation** : Récupérer et créer des commentaires dans votre application iOS

### DefaultAPI - Méthodes côté serveur

Le `DefaultAPI` contient des méthodes authentifiées qui nécessitent une clé API. Ces méthodes :
- Exigent votre clé API FastComments
- DOIVENT être appelées UNIQUEMENT depuis le code côté serveur
- Offrent un accès complet à vos données FastComments
- Sont limitées en débit par locataire

**Exemple d’utilisation** : Opérations administratives, exportation massive de données, gestion des utilisateurs

### ModerationAPI - Méthodes du tableau de bord des modérateurs

Le `ModerationAPI` fournit une suite complète d’API de modération en temps réel et rapides. Chaque méthode du `ModerationAPI` accepte un paramètre `sso` et peut s’authentifier via SSO ou un cookie de session FastComments.com.

**Exemple d’utilisation** : Créer une expérience de modération pour les modérateurs de votre communauté

**IMPORTANT** : Ne jamais exposer votre clé API dans le code côté client. Les clés API ne doivent être utilisées que côté serveur.