### Installer les dépendances

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Construire à partir des sources

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

- [Documentation de la bibliothèque du client API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### APIs publiques vs sécurisées

Pour le client API, il existe deux classes, `DefaultAPI` et `PublicAPI`. La `DefaultAPI` contient des méthodes qui requièrent votre clé API, et `PublicAPI` contient des appels d'API qui peuvent être effectués directement depuis un navigateur, un appareil mobile, etc. sans authentification.