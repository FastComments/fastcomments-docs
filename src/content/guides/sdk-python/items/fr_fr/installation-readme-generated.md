### PyPI

```bash
pip install fastcomments
```

### Contenu de la bibliothèque

Cette bibliothèque contient deux modules : le client API généré et la bibliothèque Python principale qui contient des utilitaires écrits à la main pour faciliter l'utilisation de l'API, y compris la prise en charge du SSO.

- [Documentation du client API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentation de la bibliothèque principale, y compris des exemples SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi` et `ModerationApi`. La `DefaultApi` contient des méthodes qui nécessitent votre clé API, et la `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc. sans authentification. La `ModerationApi` alimente le tableau de bord des modérateurs et contient des méthodes pour modérer les commentaires (liste, comptage, recherche, journaux, exportation), les actions de modération (supprimer/restaurer, signaler, définir l'état de revue/spam/approbation, votes, rouvrir/fermer un fil), les bannissements (bannir d'un commentaire, annuler, résumés pré-bannissement, statut/préférences de bannissement, nombre d'utilisateurs bannis), et badges et confiance (attribuer/retirer un badge, badges manuels, obtenir/définir le facteur de confiance, profil interne de l'utilisateur). Chaque méthode de la `ModerationApi` accepte un paramètre `sso` afin qu'elle puisse être appelée au nom d'un modérateur authentifié via SSO.