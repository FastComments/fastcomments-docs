### Installer les dépendances

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Compiler à partir du code source

```bash
mkdir build
cd build
cmake ..
make
```

### Installation

```bash
sudo make install
```

### Contenu de la bibliothèque

Cette bibliothèque contient le client API généré et les utilitaires SSO pour faciliter le travail avec l’API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### API publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. Le `DefaultApi` contient des méthodes qui nécessitent votre clé API, et le `PublicApi` contient  
des méthodes qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc. sans authentification. Le `ModerationApi` fournit une suite étendue d’API de modération en direct et rapides. Chaque méthode du `ModerationApi` accepte un paramètre `sso` et peut s’authentifier via SSO ou un cookie de session FastComments.com.