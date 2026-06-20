Ajoutez cette ligne au Gemfile de votre application :

```ruby
gem 'fastcomments'
```

Ensuite, exécutez :

```bash
bundle install
```

Ou installez-le vous-même comme ceci :

```bash
gem install fastcomments
```

### Contenu de la bibliothèque

Cette bibliothèque contient le client API généré et les utilitaires SSO pour faciliter l'utilisation de l'API.

- [Documentation de la bibliothèque cliente de l'API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### API publiques vs sécurisées

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi`, et `ModerationApi`. Le `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des appels API qui peuvent être effectués directement depuis un navigateur/appareil mobile/etc. sans authentification. Le `ModerationApi` contient les méthodes qui alimentent le tableau de bord du modérateur.

Le `ModerationApi` couvre la modération des commentaires (liste, comptage, recherche, journaux, export), les actions de modération (suppression/restauration, signalement, définition du statut revue/spam/approbation, votes, réouverture/fermeture de fil), les bannissements (ban depuis un commentaire, annulation, résumés avant bannissement, état/préférences de bannissement, dénombrement des utilisateurs bannis), et les badges & confiance (attribuer/supprimer un badge, badges manuels, obtenir/définir le facteur de confiance, profil interne de l'utilisateur). Chaque méthode de `ModerationApi` accepte un paramètre `sso` afin que la requête puisse être effectuée au nom d'un modérateur authentifié via SSO.