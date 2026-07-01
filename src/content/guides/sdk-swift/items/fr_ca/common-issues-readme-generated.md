### Erreurs 401 Non autorisées

Si vous obtenez des erreurs 401 lors de l'utilisation de l'API authentifiée :

1. **Vérifiez votre clé API** : assurez‑vous d’utiliser la bonne clé API depuis votre tableau de bord FastComments
2. **Vérifiez l’ID du locataire** : assurez‑vous que l’ID du locataire correspond à votre compte
3. **Format de la clé API** : la clé API doit être définie dans l’en‑tête `x-api-key` de la configuration partagée :

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Utilisation de la mauvaise API** : assurez‑vous d’utiliser `DefaultAPI` (et non `PublicAPI`) pour les appels authentifiés

### Problèmes de jeton SSO

Si les jetons SSO ne fonctionnent pas :

1. **Utilisez le mode sécurisé pour la production** : utilisez toujours `FastCommentsSSO.createSecure()` avec votre clé API en production
2. **Uniquement côté serveur** : générez des jetons SSO sécurisés sur votre serveur, n’exposez jamais votre clé API aux clients
3. **Vérifiez les données utilisateur** : assurez‑vous que tous les champs requis (id, courriel, nom d’utilisateur) sont fournis
4. **Expiration du jeton** : les jetons SSO sécurisés incluent un horodatage et peuvent expirer. Générez de nouveaux jetons au besoin.

### Erreurs SSL/TLS

Si vous rencontrez des erreurs SSL/TLS :

1. Assurez‑vous que le fichier Info.plist de votre application autorise les connexions HTTPS vers fastcomments.com
2. Vérifiez que vous n’utilisez pas d’exceptions App Transport Security qui pourraient bloquer la connexion.