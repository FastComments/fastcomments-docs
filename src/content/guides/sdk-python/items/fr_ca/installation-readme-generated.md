### Installer depuis GitHub

Installez directement à partir d'une balise de version (recommandé, entièrement reproductible) :

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Épinglez la balise plutôt qu'une branche afin que les builds soient déterministes. La même forme fonctionne dans `requirements.txt` :

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Chaque version balisée [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) possède également une roue (wheel) construite en pièce jointe si vous préférez installer directement un artefact binaire.

### Contenu de la bibliothèque

Cette bibliothèque contient deux modules : le client API généré et la bibliothèque principale Python qui contient des utilitaires écrits à la main pour faciliter l'utilisation de l'API, y compris la prise en charge du SSO.

- [Documentation du client API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentation de la bibliothèque principale, incluant des exemples SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. La classe `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc. sans authentification. La classe `ModerationApi` fournit une suite étendue d'API de modération en direct et rapides. Chaque méthode `ModerationApi` accepte un paramètre `sso` et peut s'authentifier via SSO ou un cookie de session FastComments.com.