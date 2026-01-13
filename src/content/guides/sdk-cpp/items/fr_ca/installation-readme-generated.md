### Installer les dépendances

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Compilation depuis les sources

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

Cette bibliothèque contient le client API généré et les utilitaires SSO pour faciliter le travail avec l'API.

- [Documentation de la bibliothèque cliente de l'API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### API publiques vs sécurisées

Pour le client API, il y a deux classes, `DefaultAPI` et `PublicAPI`. La `DefaultAPI` contient des méthodes qui nécessitent votre clé API, et `PublicAPI` contient des appels API qui peuvent être effectués directement depuis un navigateur/appareil mobile/etc sans authentification.