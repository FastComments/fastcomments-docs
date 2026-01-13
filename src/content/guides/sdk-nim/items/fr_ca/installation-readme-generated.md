### Utilisation de Nimble

```bash
nimble install fastcomments
```

### Compilation à partir du code source

```bash
nimble build
```

### Contenu de la bibliothèque

Cette bibliothèque contient le client API généré et les utilitaires SSO pour faciliter l'utilisation de l'API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### APIs publiques vs sécurisées

Pour le client API, il y a deux modules API, `api_default` et `api_public`. Le module `api_default` contient des méthodes qui nécessitent votre clé API, et `api_public` contient des appels d'API qui peuvent être effectués directement depuis un navigateur, un appareil mobile, etc., sans authentification.