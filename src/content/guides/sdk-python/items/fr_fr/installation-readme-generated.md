### Installer depuis GitHub

Installez directement à partir d'une balise de version (recommandé, entièrement reproductible) :

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Épinglez la balise plutôt qu'une branche afin que les builds soient déterministes. Le même format fonctionne dans `requirements.txt` :

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Chaque version balisée [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) possède également une roue compilée attachée si vous préférez installer directement un artefact binaire.

### Contenu de la bibliothèque

Cette bibliothèque contient deux modules : le client API généré et la bibliothèque principale Python qui contient des utilitaires écrits à la main pour faciliter le travail avec l'API, y compris le support SSO.

- [Documentation du client API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentation de la bibliothèque principale, incluant des exemples SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. Le `DefaultApi` contient des méthodes qui nécessitent votre clé API, et le `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc. sans authentification. Le `ModerationApi` fournit une suite étendue d'API de modération en temps réel et rapides. Chaque méthode du `ModerationApi` accepte un paramètre `sso` et peut s'authentifier via SSO ou un cookie de session FastComments.com.