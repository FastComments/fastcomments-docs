### Utilisation de Nimble

```bash
nimble install fastcomments
```

### Compilation depuis les sources

```bash
nimble build
```

### Contenu de la bibliothèque

Cette bibliothèque contient le client API généré et les utilitaires SSO pour faciliter l'utilisation de l'API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### APIs publiques vs sécurisées

Pour le client API, il y a trois modules API, `api_default`, `api_public` et `api_moderation`. Le `api_default` contient des méthodes qui nécessitent votre clé API, et `api_public` contient des appels API
qui peuvent être effectués directement depuis un navigateur/appareil mobile/etc. sans authentification. Le module `api_moderation` contient des méthodes pour le tableau de bord du modérateur.

Les méthodes de `api_moderation` couvrent la liste, le comptage, la recherche et l'export des commentaires et de leurs journaux ; les actions de modération telles que la suppression/restauration de commentaires, le signalement, la définition du statut revue/spam/approbation, l'ajustement des votes et la réouverture/fermeture de fils ; les bannissements (bannir un utilisateur d'un commentaire, annuler un bannissement, résumés pré-bannissement, statut et préférences de bannissement, et nombre d'utilisateurs bannis) ; et les badges & confiance (attribution/suppression d'un badge, liste des badges manuels, obtenir/définir le facteur de confiance d'un utilisateur, et récupération du profil interne d'un utilisateur). Chaque méthode de `api_moderation` accepte un paramètre `sso` afin que l'appel soit authentifié en tant que modérateur SSO.