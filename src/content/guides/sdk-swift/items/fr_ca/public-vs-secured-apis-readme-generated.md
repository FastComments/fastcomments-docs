Le SDK FastComments fournit deux types de points de terminaison d'API :

### PublicAPI - Points de terminaison sécurisés côté client

Le `PublicAPI` contient des points de terminaison sûrs à appeler depuis du code côté client (applications iOS/macOS). Ces points de terminaison :
- Ne nécessitent pas de clé API
- Peuvent utiliser des jetons SSO pour l'authentification
- Sont soumis à une limitation de débit par utilisateur/appareil
- Conviennent aux applications destinées aux utilisateurs finaux

**Exemple d'utilisation** : Récupération et création de commentaires dans votre application iOS

### DefaultAPI - Points de terminaison côté serveur

Le `DefaultAPI` contient des points de terminaison authentifiés qui requièrent une clé API. Ces points de terminaison :
- Exigent votre clé API FastComments
- Doivent UNIQUEMENT être appelés depuis du code côté serveur
- Fournissent un accès complet aux données FastComments
- Sont soumis à une limitation de débit par locataire

**Exemple d'utilisation** : Opérations administratives, exportation de données en masse, outils de modération

**IMPORTANT** : N'exposez jamais votre clé API dans du code côté client. Les clés API doivent être utilisées uniquement côté serveur.