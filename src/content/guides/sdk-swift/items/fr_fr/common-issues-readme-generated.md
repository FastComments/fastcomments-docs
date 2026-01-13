### Erreurs 401 (Non autorisé)

Si vous obtenez des erreurs 401 lorsque vous utilisez l'API authentifiée :

1. **Vérifiez votre API key** : Assurez-vous d'utiliser la bonne API key depuis votre tableau de bord FastComments
2. **Vérifiez le tenant ID** : Assurez-vous que le tenant ID correspond à votre compte
3. **Format de l'API key** : L'API key doit être définie sur le client API :

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Utilisation de la mauvaise API** : Assurez-vous d'utiliser `DefaultAPI` (et non `PublicAPI`) pour les appels authentifiés

### Problèmes de jetons SSO

Si les jetons SSO ne fonctionnent pas :

1. **Utilisez le mode sécurisé en production** : Utilisez toujours `FastCommentsSSO.createSecure()` avec votre API key en production
2. **Côté serveur uniquement** : Générez les jetons SSO sécurisés sur votre serveur, n'exposez jamais votre API key aux clients
3. **Vérifiez les données utilisateur** : Assurez-vous que tous les champs requis (id, email, username) sont fournis
4. **Expiration du token** : Les jetons SSO sécurisés incluent un horodatage et peuvent expirer. Générez de nouveaux jetons selon les besoins.

### Erreurs SSL/TLS

Si vous rencontrez des erreurs SSL/TLS :

1. Assurez-vous que l'Info.plist de votre application autorise les connexions HTTPS vers fastcomments.com
2. Vérifiez que vous n'utilisez pas d'exceptions App Transport Security susceptibles de bloquer la connexion