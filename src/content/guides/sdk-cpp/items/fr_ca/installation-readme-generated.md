### Installer les dépendances

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Compilation à partir des sources

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

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### APIs publiques vs sécurisées

For the API client, there are three classes, `DefaultApi`, `PublicApi`, and `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains
méthodes qui peuvent être effectuées directement depuis un navigateur/appareil mobile/etc sans authentification. The `ModerationApi` contains methods that power the moderator dashboard - listing,
counting, searching, exporting and pulling logs for comments, moderation actions (supprimer/restaurer, signaler, définir l'état examen/spam/approbation, ajuster les votes, rouvrir/fermer les fils),
bans (bannir d'un commentaire, annuler des bannissements, résumés pré-bannissement, état et préférences de bannissement, comptes d'utilisateurs bannis), and badges & trust (attribuer/retirer des badges, badges manuels, obtenir/définir le facteur de confiance, profil interne de l'utilisateur). Every `ModerationApi` method accepts an `sso` parameter so the call is performed on behalf of an SSO-authenticated moderator.