### Installer depuis GitHub

Installez directement à partir d’une balise de version (recommandé, entièrement reproductible) :

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Épinglez la balise plutôt qu’une branche afin que les builds soient déterministes. La même forme fonctionne dans `requirements.txt` :

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Chaque [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) balisée possède également une roue pré‑construite jointe si vous préférez installer un artefact binaire directement.

### Contenu de la bibliothèque

Cette bibliothèque contient deux modules : le client API généré et la bibliothèque centrale Python qui comprend des utilitaires écrits à la main pour faciliter l’utilisation de l’API, y compris la prise en charge du SSO.

- [Documentation du client API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentation de la bibliothèque centrale, y compris des exemples SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. La classe `DefaultApi` contient des méthodes qui requièrent votre clé API, et `PublicApi` contient des méthodes pouvant être appelées directement depuis un navigateur, un appareil mobile, etc., sans authentification. La classe `ModerationApi` fournit une suite étendue d’API de modération en direct et rapides. Chaque méthode `ModerationApi` accepte un paramètre `sso` et peut s’authentifier via SSO ou un cookie de session FastComments.com.