### Installer depuis GitHub

Installez directement à partir d'une balise de version (recommandé, entièrement reproductible) :

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Épinglez la balise plutôt qu'une branche afin que les builds soient déterministes. Le même format fonctionne dans `requirements.txt` :

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Chaque version balisée [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) possède également une roue (wheel) construite en pièce jointe si vous préférez installer directement un artefact binaire.

### Contenu de la bibliothèque

Cette bibliothèque contient deux modules : le client API généré et la bibliothèque Python principale qui contient des utilitaires écrits à la main pour faciliter l'utilisation de l'API, y compris la prise en charge du SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. Le `DefaultApi` contient des méthodes qui nécessitent votre clé API, et le `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc sans authentification. Le `ModerationApi` fournit une suite étendue d'API de modération en temps réel et rapides. Chaque méthode du `ModerationApi` accepte un paramètre `sso` et peut s'authentifier via SSO ou un cookie de session FastComments.com.