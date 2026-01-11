### PyPI

```bash
pip install fastcomments
```

### Contenu de la bibliothèque

Cette bibliothèque contient deux modules : le client API généré et la bibliothèque Python principale qui contient des utilitaires écrits à la main pour faciliter l'utilisation de l'API, y compris le support SSO.

- [Documentation du client API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentation de la bibliothèque principale, y compris des exemples SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### APIs publiques vs sécurisées

Pour le client API, il existe deux classes, `DefaultApi` et `PublicApi`. La `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des appels d'API qui peuvent être effectués directement depuis un navigateur, un appareil mobile, etc., sans authentification.