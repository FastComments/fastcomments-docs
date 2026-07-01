### Installer les dépendances

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Compilation depuis le source

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

Cette bibliothèque contient le client API généré ainsi que les utilitaires SSO pour faciliter l'utilisation de l'API.

- [Documentation de la bibliothèque client API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### API publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. La classe `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc. sans authentification. La classe `ModerationApi` fournit une suite étendue d'API de modération en temps réel et rapides. Chaque méthode de `ModerationApi` accepte un paramètre `sso` et peut s'authentifier via SSO ou un cookie de session FastComments.com.