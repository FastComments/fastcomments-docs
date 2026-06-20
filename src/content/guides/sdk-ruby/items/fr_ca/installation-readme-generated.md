Ajoutez cette ligne au Gemfile de votre application :

```ruby
gem 'fastcomments'
```

Ensuite, exécutez :

```bash
bundle install
```

Ou installez-le vous-même :

```bash
gem install fastcomments
```

### Contenu de la bibliothèque

Cette bibliothèque contient le client API généré et les utilitaires SSO pour faciliter le travail avec l'API.

- [Documentation de la bibliothèque cliente API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### API publiques vs API sécurisées

Pour le client API, il existe trois classes : `DefaultApi`, `PublicApi`, et `ModerationApi`. La `DefaultApi` contient les méthodes qui nécessitent votre clé API, et `PublicApi` contient des appels d'API
qui peuvent être effectués directement depuis un navigateur/appareil mobile/etc. sans authentification. La `ModerationApi` contient les méthodes qui alimentent le tableau de bord du modérateur.

La `ModerationApi` couvre la modération des commentaires (liste, comptage, recherche, journaux, export), les actions de modération (supprimer/restaurer, signaler, définir l'état examen/spam/approbation, votes, rouvrir/fermer le fil),
les bannissements (bannir à partir d'un commentaire, annuler, résumés avant bannissement, statut/préférences de bannissement, décomptes d'utilisateurs bannis), et les badges & confiance (attribuer/supprimer un badge, badges manuels, obtenir/définir le facteur de confiance, profil interne de l'utilisateur).
Chaque méthode de `ModerationApi` accepte un paramètre `sso` afin que la requête puisse être effectuée au nom d'un modérateur authentifié via SSO.