---
Le FastComments SDK fournit deux types de points de terminaison d'API :

### PublicAPI - Points de terminaison sécurisés côté client

Le `PublicAPI` contient des points de terminaison qui sont sûrs à appeler depuis du code côté client (applications iOS/macOS). Ces points de terminaison :
- Ne nécessitent pas d'API key
- Peuvent utiliser des SSO tokens pour l'authentification
- Sont soumis à des limites de fréquence par utilisateur/appareil
- Conviennent aux applications destinées aux utilisateurs finaux

**Exemple d'utilisation** : Récupération et création de commentaires dans votre application iOS

### DefaultAPI - Points de terminaison côté serveur

Le `DefaultAPI` contient des points de terminaison authentifiés qui requièrent une API key. Ces points de terminaison :
- Exigent votre FastComments API key
- Doivent UNIQUEMENT être appelés depuis du code côté serveur
- Offrent un accès complet aux données FastComments
- Sont soumis à des limites de fréquence par tenant

**Exemple d'utilisation** : Opérations administratives, exportation de données en masse, outils de modération

**IMPORTANT** : N'exposez jamais votre API key dans du code côté client. Les API keys ne doivent être utilisées que côté serveur.
---