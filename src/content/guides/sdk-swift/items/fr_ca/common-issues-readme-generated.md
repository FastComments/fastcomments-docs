### 401 Erreurs non autorisées

If you're getting 401 errors when using the authenticated API:

1. **Vérifiez votre clé API**: Assurez-vous d'utiliser la clé API correcte depuis votre tableau de bord FastComments
2. **Vérifiez l'ID du locataire**: Assurez-vous que l'ID du locataire correspond à votre compte
3. **Format de la clé API**: La clé API doit être définie sur le client API:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Utilisation d'une API incorrecte**: Assurez-vous d'utiliser `DefaultAPI` (et non `PublicAPI`) pour les appels authentifiés

### Problèmes de jeton SSO

If SSO tokens aren't working:

1. **Utilisez le mode sécurisé en production**: Utilisez toujours `FastCommentsSSO.createSecure()` avec votre clé API en production
2. **Côté serveur uniquement**: Générez les jetons SSO sécurisés sur votre serveur, n'exposez jamais votre clé API aux clients
3. **Vérifiez les données utilisateur**: Assurez-vous que tous les champs requis (id, email, username) sont fournis
4. **Expiration du jeton**: Les jetons SSO sécurisés incluent un horodatage et peuvent expirer. Générez des jetons neufs au besoin.

### Erreurs SSL/TLS

If you encounter SSL/TLS errors:

1. Assurez-vous que le fichier Info.plist de votre application autorise les connexions HTTPS vers fastcomments.com
2. Vérifiez que vous n'utilisez pas d'exceptions App Transport Security qui pourraient bloquer la connexion