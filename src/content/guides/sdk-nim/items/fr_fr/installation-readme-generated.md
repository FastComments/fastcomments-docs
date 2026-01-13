### Utilisation de Nimble

```bash
nimble install fastcomments
```

### Compilation depuis les sources

```bash
nimble build
```

### Contenu de la bibliothèque

Cette bibliothèque contient le client API généré et les utilitaires SSO pour faciliter l'utilisation de l'API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### API publiques vs sécurisées

Pour le client API, il existe deux modules API, `api_default` et `api_public`. Le `api_default` contient des méthodes qui nécessitent votre clé API, et `api_public` contient des appels d'API
qui peuvent être effectués directement depuis un navigateur/appareil mobile/etc sans authentification.