---
Ajoutez cette ligne au Gemfile de votre application :

```ruby
gem 'fastcomments'
```

Et ensuite exécutez :

```bash
bundle install
```

Ou installez-le vous-même avec :

```bash
gem install fastcomments
```

### Contenu de la bibliothèque

Cette bibliothèque contient le client API généré et les utilitaires SSO pour faciliter l'utilisation de l'API.

- [Documentation de la bibliothèque cliente API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### API publiques vs sécurisées

Pour le client API, il existe deux classes, `DefaultApi` et `PublicApi`. Le `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des appels d'API
qui peuvent être effectués directement depuis un navigateur/appareil mobile/etc sans authentification.
---