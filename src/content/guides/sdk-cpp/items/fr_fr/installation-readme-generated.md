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

Cette bibliothèque contient le client API généré et les utilitaires SSO pour faciliter l'utilisation de l'API.

- [Documentation de la bibliothèque cliente de l'API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### API publiques vs API sécurisées

For the API client, there are three classes, `DefaultApi`, `PublicApi`, and `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains
methods that can be made directly from a browser/mobile device/etc without authentication. The `ModerationApi` contains methods that power the moderator dashboard - listing,
counting, searching, exporting and pulling logs for comments, moderation actions (supprimer/restaurer, signaler, définir le statut à revoir/spam/approuvé, ajuster les votes, réouvrir/fermer les fils),
bans (bannissements à partir d'un commentaire, annuler des bannissements, résumés pré-bannissement, statut et préférences de bannissement, dénombrements d'utilisateurs bannis), and badges & trust (attribuer/retirer des badges, badges manuels, obtenir/définir le facteur de confiance,
profil interne de l'utilisateur). Every `ModerationApi` method accepts an `sso` parameter so the call is performed on behalf of an SSO-authenticated moderator.