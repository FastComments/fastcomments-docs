### PyPI

```bash
pip install fastcomments
```

### Contenu de la bibliothèque

Cette bibliothèque contient deux modules : le client API généré et la bibliothèque Python principale qui contient des utilitaires écrits à la main pour faciliter l'utilisation de l'API, y compris le support SSO.

- [Docs de la bibliothèque cliente de l'API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Docs de la bibliothèque principale, y compris des exemples SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API publiques vs sécurisées

Pour le client API, il y a trois classes : `DefaultApi`, `PublicApi` et `ModerationApi`. `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc. sans authentification. `ModerationApi` alimente le tableau de bord des modérateurs et contient des méthodes pour modérer les commentaires (liste, compte, recherche, journaux, export), les actions de modération (supprimer/restaurer, signaler, définir l'état examen/spam/approbation, votes, rouvrir/fermer le fil), les interdictions (interdire via un commentaire, annuler, résumés avant interdiction, état/préférences d'interdiction, nombre d'utilisateurs bannis), et les badges et la confiance (attribuer/retirer un badge, badges manuels, obtenir/définir le facteur de confiance, profil interne de l'utilisateur). Chaque méthode de `ModerationApi` accepte un paramètre `sso` afin qu'elle puisse être appelée au nom d'un modérateur authentifié via SSO.