### Utiliser Nimble

```bash
nimble install fastcomments
```

### Compilation depuis le code source

```bash
nimble build
```

### Contenu de la bibliothèque

Cette bibliothèque contient le client API généré et les utilitaires SSO pour faciliter l’utilisation de l’API.

- [Docs de la bibliothèque cliente API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### APIs publiques vs sécurisées

Pour le client API, il existe trois modules API : `api_default`, `api_public` et `api_moderation`. Le module `api_default` contient des méthodes qui nécessitent votre clé API, et `api_public` contient des appels API qui peuvent être effectués directement depuis un navigateur, un appareil mobile, etc., sans authentification. Le module `api_moderation` contient des méthodes pour le tableau de bord du modérateur.

Le module `api_moderation` fournit une suite étendue d’APIs de modération en temps réel et rapides. Chaque méthode `api_moderation` accepte un paramètre `sso` et peut s’authentifier via SSO ou un cookie de session FastComments.com.